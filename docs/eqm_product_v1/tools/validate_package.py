#!/usr/bin/env python3
"""Validate this handoff's structure and original delivery hashes, never EQM code.

Requires Python 3.9+ and only the standard library. No network, source imports,
subprocesses, writes, or execution of repository instructions are performed.
Use --structure-only after intentional implementation edits. Hashes detect
changes relative to the delivered list; they are not a digital signature.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path, PurePosixPath
from urllib.parse import unquote, urlsplit

REQUIRED = [
    'PROMPT.md', 'summary.txt', 'README.md', 'HANDOFF_MANIFEST.json',
    'specs/PRODUCT_SPEC.md', 'specs/ARCHITECTURE.md', 'specs/API_CONTRACTS.md',
    'specs/DATA_MODEL.md', 'specs/ACCEPTANCE_CRITERIA.md',
    'implementation/COMMIT_SEQUENCE.md', 'implementation/TEST_PLAN.md',
    'implementation/VERIFICATION.md', 'implementation/OPEN_QUESTIONS.md',
    'implementation/STEP_REPORT_TEMPLATE.md', 'implementation/DEVIATIONS.md',
    'implementation/STATUS.md', 'implementation/requirements.json',
    'implementation/steps.json', 'implementation/optional_steps.json',
    'implementation/acceptance.json', 'implementation/traceability.json',
    'implementation/progress.json', 'implementation/findings.json',
    'implementation/open_questions.json', 'decisions/ADR-0001-architecture.md',
    'repo/AGENTS.md', 'repo/BASELINE.md', 'repo/REVIEW_FINDINGS.md',
    'sources/SOURCES.md', 'sources/provenance.json', 'tools/validate_package.py',
]
STEP_FIELDS = ['title', 'purpose', 'scope', 'files', 'tests',
               'verification_commands', 'expected', 'commit_message']
SECTION_TITLES = [
    '### 1. Step title', '### 2. Purpose', '### 3. Exact scope of code changes',
    '### 4. Files/modules likely involved',
    '### 5. Unit tests or integration tests required',
    '### 6. Verification commands', '### 7. Expected result',
    '### 8. Commit message',
]


def contained(root: Path, path: Path) -> bool:
    try:
        path.resolve().relative_to(root.resolve())
        return True
    except ValueError:
        return False


def ids(items, label: str, errors: list[str]) -> set[str]:
    result: set[str] = set()
    for item in items:
        key = item.get('id')
        if not isinstance(key, str) or not key or key in result:
            errors.append(f'{label}: absent/duplicate ID {key!r}')
        else:
            result.add(key)
    return result


def normalized_inline(text: str) -> str:
    # The full plan and chapter copies differ only in relative link paths.
    return re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', text).strip()


def sections(text: str) -> dict[str, str]:
    matches = list(re.finditer(r'^<a id="([sn]\d{3})"></a>\s*$', text, re.M))
    result = {}
    for i, match in enumerate(matches):
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        result[match.group(1).upper()] = text[match.end():end].strip()
    return result


def known_anchors(text: str) -> set[str]:
    result = set(re.findall(r'<a\s+id=[\'"]([^\'"]+)[\'"]', text))
    seen: dict[str, int] = {}
    for heading in re.findall(r'^#{1,6}\s+(.+)$', text, re.M):
        heading = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', heading)
        slug = re.sub(r'[^\w\-\s]', '', heading.lower()).replace(' ', '-')
        count = seen.get(slug, 0)
        seen[slug] = count + 1
        result.add(slug if not count else f'{slug}-{count}')
    return result


def validate(root: Path, structure_only: bool) -> dict:
    errors: list[str] = []
    root = root.resolve()
    for name in REQUIRED:
        if not (root / name).is_file():
            errors.append(f'Missing required file: {name}')
    if errors:
        return {'ok': False, 'scope': 'handoff_only', 'errors': errors}

    files = sorted(p for p in root.rglob('*') if p.is_file())
    for p in root.rglob('*'):
        if p.is_symlink():
            errors.append(f'Symlink forbidden in package: {p.relative_to(root)}')
    if errors:
        return {'ok': False, 'scope': 'handoff_only', 'errors': errors}
    data = {}
    for p in files:
        if p.suffix == '.json':
            try:
                data[p.relative_to(root).as_posix()] = json.loads(p.read_text(encoding='utf-8'))
            except (UnicodeError, json.JSONDecodeError) as exc:
                errors.append(f'Invalid JSON {p.relative_to(root)}: {exc}')
    if errors:
        return {'ok': False, 'scope': 'handoff_only', 'errors': errors}

    manifest = data['HANDOFF_MANIFEST.json']
    reqs = data['implementation/requirements.json']
    steps = data['implementation/steps.json']['steps']
    optional = data['implementation/optional_steps.json']['steps']
    acceptance = data['implementation/acceptance.json']
    findings = data['implementation/findings.json']
    questions = data['implementation/open_questions.json']
    trace = data['implementation/traceability.json']
    progress = data['implementation/progress.json']
    regressions = data.get('implementation/review_regressions.json', [])
    cycles = data['implementation/steps.json'].get('cycles', [])
    req_ids = ids(reqs, 'requirements', errors)
    ac_ids = ids(acceptance, 'acceptance', errors)
    step_ids = ids(steps, 'mandatory steps', errors)
    opt_ids = ids(optional, 'optional steps', errors)
    finding_ids = ids(findings, 'findings', errors)
    question_ids = ids(questions, 'open decisions', errors)
    regression_ids = ids(regressions, 'review regressions', errors)
    cycle_ids = ids(cycles, 'execution cycles', errors)
    counts = {
        'mandatory_steps': len(steps),
        'mandatory_chapters': len(set(s['chapter'] for s in steps)),
        'optional_native_steps': len(optional),
        'requirements': len(reqs),
        'acceptance_cases': len(acceptance),
        'review_findings': len(findings),
        'open_decision_gates': len(questions),
        'architecture_decision_records': len(list((root/'decisions').glob('ADR-*.md'))),
    }
    if manifest.get('package_revision', 1) >= 2:
        counts['review_regression_clauses'] = len(regressions)
        counts['execution_cycles'] = len(cycles)
        if not regressions or not cycles:
            errors.append('Approved revision lacks regression clauses or execution cycles')
        approval = data['implementation/steps.json'].get('approval', {})
        if approval.get('status') != 'approved_for_plan_materialization':
            errors.append('Approved revision lacks its owner approval record')
        if progress.get('approval') != approval:
            errors.append('Progress/plan approval records disagree')
        if progress.get('plan_revision') != manifest['package_revision']:
            errors.append('Progress/manifest plan revisions disagree')
        if data['implementation/steps.json'].get('revision') != manifest['package_revision']:
            errors.append('Machine plan/manifest revisions disagree')
    if counts != manifest.get('counts'):
        errors.append(f'Manifest counts disagree: observed {counts}')
    if [s['id'] for s in steps] != [f'S{i:03d}' for i in range(1, len(steps)+1)]:
        errors.append('Mandatory step IDs/order are not consecutive S001 onward')
    if [s['id'] for s in optional] != [f'N{i:03d}' for i in range(1, len(optional)+1)]:
        errors.append('Optional step IDs/order are not consecutive N001 onward')
    if data['implementation/steps.json'].get('step_order') != [s['id'] for s in steps]:
        errors.append('Machine plan step_order disagrees with its step records')

    if cycles:
        if [c['id'] for c in cycles] != [f'EQM-P{i:02d}' for i in range(1, len(cycles)+1)]:
            errors.append('Execution cycle IDs/order are not consecutive')
        if [sid for c in cycles for sid in c.get('steps', [])] != [s['id'] for s in steps]:
            errors.append('Execution cycles do not partition the ordered mandatory steps')
        for i, cycle in enumerate(cycles):
            for field in ['title', 'steps', 'definition_of_green', 'verification_lane', 'document', 'gates']:
                if not cycle.get(field):
                    errors.append(f"{cycle['id']}: missing cycle field {field}")
            members = cycle.get('steps', [])
            if not members or cycle.get('first_step') != members[0] or cycle.get('last_step') != members[-1]:
                errors.append(f"{cycle['id']}: cycle bounds disagree")
            if cycle.get('predecessor') != (cycles[i-1]['id'] if i else 'PREFLIGHT'):
                errors.append(f"{cycle['id']}: cycle predecessor disagrees")
            if not set(cycle.get('gates', [])) <= question_ids:
                errors.append(f"{cycle['id']}: unknown decision gate")
            document = root / cycle.get('document', '')
            if not contained(root, document) or not document.is_file():
                errors.append(f"{cycle['id']}: missing cycle document")
            else:
                cycle_text = document.read_text(encoding='utf-8')
                if cycle['id'] not in cycle_text or cycle['definition_of_green'] not in cycle_text:
                    errors.append(f"{cycle['id']}: cycle document omits identity/green definition")
        progress_cycles = progress.get('cycles', [])
        if [c['id'] for c in progress_cycles] != [c['id'] for c in cycles]:
            errors.append('Progress cycle inventory/order disagrees')
        for declared, tracked in zip(cycles, progress_cycles):
            if any(declared[k] != tracked.get(k) for k in ['id', 'first_step', 'last_step']):
                errors.append(f"{declared['id']}: progress cycle bounds disagree")
            actual_completed = sum(s.get('status') == 'complete' for s in progress['steps'] if s['id'] in declared['steps'])
            if tracked.get('total_steps') != len(declared['steps']) or tracked.get('completed_steps') != actual_completed:
                errors.append(f"{declared['id']}: progress cycle completion counts disagree")
        if sum(s.get('status') == 'in_progress' for s in progress['steps']) > 1:
            errors.append('More than one implementation step is active')

    for regression in regressions:
        rid = regression['id']
        for field in ['title', 'findings', 'requirements', 'acceptance', 'steps', 'source_commit', 'source_anchors', 'given', 'when', 'expected', 'lane', 'status']:
            if not regression.get(field):
                errors.append(f'{rid}: missing regression field {field}')
        for field, valid in [('findings', finding_ids), ('requirements', req_ids), ('acceptance', ac_ids), ('steps', step_ids)]:
            if not set(regression.get(field, [])) <= valid:
                errors.append(f'{rid}: unknown {field} reference')
        for anchor in regression.get('source_anchors', []):
            path = PurePosixPath(anchor.get('path', ''))
            if str(path) == '.' or path.is_absolute() or '..' in path.parts or not isinstance(anchor.get('line'), int) or anchor['line'] < 1:
                errors.append(f'{rid}: invalid repository-relative source anchor')
    if regressions:
        if [r['id'] for r in regressions] != [f'RA{i:02d}' for i in range(1, len(regressions)+1)]:
            errors.append('Review regression IDs/order are not consecutive')
        if [r['id'] for r in progress.get('review_regressions', [])] != [r['id'] for r in regressions]:
            errors.append('Progress regression inventory/order disagrees')
        for items, field in [(steps, 'steps'), (findings, 'findings'), (acceptance, 'acceptance')]:
            for item in items:
                expected = [r['id'] for r in regressions if item['id'] in r[field]]
                if item.get('review_regressions') != expected:
                    errors.append(f"{item['id']}: review regression mapping disagrees")
        by_step = {s['id']: s for s in steps}
        for regression in regressions:
            for sid in regression['steps']:
                if sid in by_step and (not set(regression['requirements']) <= set(by_step[sid]['requirements']) or
                                       not set(regression['acceptance']) <= set(by_step[sid]['acceptance'])):
                    errors.append(f"{regression['id']}: owning step {sid} omits mapped contracts")
        review_text = (root/'repo/REVIEW_FINDINGS.md').read_text(encoding='utf-8')
        for regression in regressions:
            marker = f"### {regression['id']} — {regression['title']}"
            if marker not in review_text:
                errors.append(f"{regression['id']}: Markdown regression section missing")
                continue
            body = review_text.split(marker, 1)[1].split('\n### ', 1)[0]
            for field in ['given', 'when', 'expected']:
                if regression[field] not in body:
                    errors.append(f"{regression['id']}: Markdown/machine regression mismatch in {field}")

    full_text = (root/'implementation/COMMIT_SEQUENCE.md').read_text(encoding='utf-8')
    optional_text = (root/'implementation/OPTIONAL_NATIVE_COMMIT_SEQUENCE.md').read_text(encoding='utf-8')
    full_sections = sections(full_text)
    for cycle in cycles:
        if cycle['id'] not in full_text or cycle['definition_of_green'] not in full_text:
            errors.append(f"{cycle['id']}: governing plan omits cycle identity/green definition")
    optional_sections = sections(optional_text)
    if set(full_sections) != step_ids:
        errors.append('Full Markdown plan does not contain exactly the mandatory step sections')
    if set(optional_sections) != opt_ids:
        errors.append('Optional Markdown plan does not contain exactly the optional step sections')
    chapter_sections = {}
    chapter_files = sorted((root/'implementation/sequence').glob('*.md'))
    for p in chapter_files:
        for key, value in sections(p.read_text(encoding='utf-8')).items():
            if key in chapter_sections:
                errors.append(f'Duplicate chapter step: {key}')
            chapter_sections[key] = value
    if set(chapter_sections) != step_ids:
        errors.append('Chapter copies do not cover exactly the mandatory sequence')
    if len(chapter_files) != counts['mandatory_chapters']:
        errors.append('Chapter file count disagrees with planned chapters')

    for sequence, rendered, prefix in [(steps, full_sections, 'S'), (optional, optional_sections, 'N')]:
        for i, step in enumerate(sequence):
            sid = step['id']
            for key in STEP_FIELDS:
                if not step.get(key):
                    errors.append(f'{sid}: missing required step field {key}')
            if i and step.get('predecessor') != sequence[i-1]['id']:
                errors.append(f'{sid}: predecessor is not previous ordered step')
            if not set(step.get('requirements', [])) <= req_ids:
                errors.append(f'{sid}: unknown requirement reference')
            if not set(step.get('acceptance', [])) <= ac_ids:
                errors.append(f'{sid}: unknown acceptance reference')
            for contract in step.get('contract_files', []):
                p = root / contract
                if not contained(root, p) or not p.is_file():
                    errors.append(f'{sid}: missing contract {contract}')
            commands = step.get('verification_commands', [])
            for expected in ['cargo check', 'cargo test', 'cargo fmt', 'cargo clippy', 'cargo xtask check']:
                if not any(c.startswith(expected) for c in commands):
                    errors.append(f'{sid}: required verification category absent: {expected}')
            if prefix == 'S' and i < 6 and any('-p eqm_application' in c for c in commands):
                errors.append(f'{sid}: targets application package before its creation')
            if not re.fullmatch(r'(?:build|chore|docs|feat|fix|refactor|test|perf): [^\n]+', step.get('commit_message', '')):
                errors.append(f'{sid}: commit subject violates documented convention')
            body = rendered.get(sid, '')
            for title in SECTION_TITLES:
                if title not in body:
                    errors.append(f'{sid}: missing Markdown step field {title}')
            for field in ['title', 'purpose', 'scope', 'tests', 'expected', 'commit_message']:
                if step.get(field, '') not in body:
                    errors.append(f'{sid}: Markdown/machine-plan mismatch in {field}')
            for command in commands:
                if command not in body:
                    errors.append(f'{sid}: command omitted in Markdown: {command}')
            if prefix == 'S' and sid in chapter_sections:
                if normalized_inline(body) != normalized_inline(chapter_sections[sid]):
                    errors.append(f'{sid}: full/chapter copy disagreement')
            for q in re.findall(r'\bQ\d{2}\b', step.get('gate') or ''):
                if q not in question_ids:
                    errors.append(f'{sid}: unknown gate {q}')

    trace_by_id = {t.get('requirement'): t for t in trace}
    if len(trace_by_id) != len(trace) or set(trace_by_id) != req_ids:
        errors.append('Traceability does not uniquely cover every requirement')
    for req in reqs:
        rid = req['id']
        if not (root/req['contract']).is_file():
            errors.append(f'{rid}: contract file missing')
        tr = trace_by_id.get(rid, {})
        expected_steps = [s['id'] for s in steps if rid in s['requirements']]
        expected_opt = [s['id'] for s in optional if rid in s['requirements']]
        expected_ac = [a['id'] for a in acceptance if rid in a['requirements']]
        expected_findings = [f['id'] for f in findings if rid in f['requirements']]
        if not expected_steps:
            errors.append(f'{rid}: no mandatory contribution/disposition step')
        for field, value in [('steps', expected_steps), ('optional_steps', expected_opt),
                             ('acceptance', expected_ac), ('findings', expected_findings)]:
            if tr.get(field) != value:
                errors.append(f'{rid}: traceability disagreement for {field}')
        if regressions and tr.get('review_regressions') != [r['id'] for r in regressions if rid in r['requirements']]:
            errors.append(f'{rid}: traceability disagreement for review regressions')
        if not expected_ac:
            errors.append(f'{rid}: no acceptance criterion')
    for ac in acceptance:
        for field in ['title', 'given', 'when', 'expected', 'lane']:
            if not ac.get(field):
                errors.append(f"{ac['id']}: missing acceptance field {field}")
        if not set(ac['requirements']) <= req_ids:
            errors.append(f"{ac['id']}: unknown requirement")
    for f in findings:
        if not set(f['requirements']) <= req_ids:
            errors.append(f"{f['id']}: unknown requirement")
    if [s['id'] for s in progress['steps']] != [s['id'] for s in steps]:
        errors.append('Progress ledger step order differs from plan')
    if {a['id'] for a in progress['acceptance']} != ac_ids:
        errors.append('Progress ledger acceptance IDs differ from contracts')

    # Only original delivery validation asserts initial untouched statuses.
    if not structure_only:
        if any(s.get('initial_status') != 'pending' for s in steps):
            errors.append('Delivered mandatory plan does not start pending')
        if any(s.get('initial_status') != 'not_activated' for s in optional):
            errors.append('Delivered optional extension is not explicitly inactive')
        if any(a.get('status') != 'not_run' for a in acceptance):
            errors.append('Delivered acceptance must not claim product tests ran')
        if any(s.get('status') != 'pending' or s.get('commit_sha') is not None for s in progress['steps']):
            errors.append('Delivered progress must not claim implementation commits')
        if any(f.get('status') != 'open_revalidate' for f in findings):
            errors.append('Delivered static findings must remain open for source revalidation')
        if any(r.get('status') != 'not_run' or r.get('evidence') for r in regressions):
            errors.append('Delivered regression clauses must not claim implementation evidence')
        if any(c.get('status') != 'pending' or c.get('completed_steps') != 0 for c in progress.get('cycles', [])):
            errors.append('Delivered execution cycles must remain pending')

    anchor_cache = {}
    link_count = 0
    for p in files:
        if p.suffix != '.md':
            continue
        text = p.read_text(encoding='utf-8')
        if len(re.findall(r'^```', text, re.M)) % 2:
            errors.append(f'Unbalanced Markdown code fences: {p.relative_to(root)}')
        for match in re.finditer(r'(?<!!)\[[^\]]+\]\(([^\s)]+)(?:\s+"[^"]*")?\)', text):
            dest = match.group(1)
            parsed = urlsplit(dest)
            if parsed.scheme or parsed.netloc:
                continue
            link_count += 1
            target = (p.parent/unquote(parsed.path)).resolve() if parsed.path else p
            if not contained(root, target) or not target.exists():
                errors.append(f'Broken/escaping link {p.relative_to(root)} -> {dest}')
                continue
            if parsed.fragment and target.suffix == '.md':
                if target not in anchor_cache:
                    anchor_cache[target] = known_anchors(target.read_text(encoding='utf-8'))
                if unquote(parsed.fragment) not in anchor_cache[target]:
                    errors.append(f'Broken anchor {p.relative_to(root)} -> {dest}')

    summary = (root/'summary.txt').read_text(encoding='utf-8')
    for key in req_ids | ac_ids | step_ids | opt_ids | finding_ids | question_ids | regression_ids | cycle_ids:
        if not re.search(r'(?<![A-Za-z0-9])'+re.escape(key)+r'(?![A-Za-z0-9])', summary):
            errors.append(f'Standalone reconstruction omits {key}')
    for required_text in ['not_run', 'not_activated', 'not_granted', 'not_started']:
        if required_text not in json.dumps(manifest.get('initial_state')):
            errors.append(f'Manifest initial state omits {required_text}')

    hash_count = 0
    if not structure_only:
        checksum_path = root/'SHA256SUMS'
        if not checksum_path.is_file():
            errors.append('Missing original payload checksum list; use --structure-only during authoring')
        else:
            recorded: dict[str, str] = {}
            for n, line in enumerate(checksum_path.read_text(encoding='utf-8').splitlines(), 1):
                m = re.fullmatch(r'([0-9a-f]{64})  (.+)', line)
                if not m:
                    errors.append(f'Invalid checksum line {n}')
                    continue
                digest, name = m.groups()
                rel = PurePosixPath(name)
                if rel.is_absolute() or '..' in rel.parts or name in recorded or name == 'SHA256SUMS':
                    errors.append(f'Unsafe/duplicate checksum path: {name}')
                    continue
                recorded[name] = digest
            actual_names = {p.relative_to(root).as_posix() for p in files if p.name != 'SHA256SUMS'}
            if set(recorded) != actual_names:
                errors.append(f'Checksum inventory differs: missing={sorted(actual_names-set(recorded))}; extra={sorted(set(recorded)-actual_names)}')
            for name, digest in recorded.items():
                p = root/name
                if not contained(root, p) or not p.is_file():
                    errors.append(f'Missing/escaping hashed payload: {name}')
                    continue
                observed = hashlib.sha256(p.read_bytes()).hexdigest()
                if observed != digest:
                    errors.append(f'Payload hash mismatch: {name}')
                hash_count += 1
            if not (root/'PACKAGE_VALIDATION.json').is_file():
                errors.append('Original delivery lacks package validation record')

    return {
        'ok': not errors, 'scope': 'handoff_document_structure_and_integrity_only',
        'mode': 'structure_only' if structure_only else 'original_delivery',
        'package_revision': manifest.get('package_revision', 1),
        'files': len(files), 'json_files': len(data), 'internal_markdown_links': link_count,
        'hashed_payloads': hash_count, 'counts': counts,
        'rust_tests_executed': False, 'native_tests_executed': False,
        'signature_or_deployment_authority_established': False,
        'errors': errors,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1],
                        help='Extracted package root (default: parent of tools/)')
    parser.add_argument('--structure-only', action='store_true',
                        help='Check structure/traceability, not original delivery hashes/statuses')
    parser.add_argument('--json', action='store_true', help='Emit a JSON report to stdout')
    args = parser.parse_args()
    try:
        report = validate(args.root, args.structure_only)
    except (OSError, ValueError, KeyError, TypeError) as exc:
        report = {'ok': False, 'scope': 'handoff_only',
                  'errors': [f'Invalid package or unreadable input: {type(exc).__name__}: {exc}']}
    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    elif report['ok']:
        print('PASS — handoff structure' + (' and original payload checksums' if not args.structure_only else ''))
        print(json.dumps({k: report[k] for k in ['files', 'internal_markdown_links', 'hashed_payloads', 'counts']}, indent=2))
        print('No EQM Rust/native tests, signing or deployment were executed by this validator.')
    else:
        print('FAIL — handoff package validation', file=sys.stderr)
        for error in report['errors']:
            print('  '+error, file=sys.stderr)
    return 0 if report['ok'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
