package fixture.signup

data class SignupState(val email: String = "")

fun SignupScreen(state: SignupState): String =
    if (state.email.isEmpty()) "Enter email" else "Continue"
