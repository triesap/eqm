import SwiftUI

struct SignupView: View {
    @State private var email = ""

    var body: some View {
        Form {
            TextField("Email", text: $email).textContentType(.emailAddress)
            NavigationLink("Continue", value: "otp").disabled(email.isEmpty)
        }
    }
}
