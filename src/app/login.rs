use leptos::{component, view, IntoView};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="login">
            <img src="logo.svg" alt=""/>
            <h1>"Sign in to Enzyme"</h1>
            <form class="inputbox" action="/login" method="POST">
                <label for="username">
                    <b>"Username"</b> <br/>
                    <input type="text" name="username" required/>
                </label>
                <div class="password-wrapper">
                    <label for="password">
                        <b> "Password"</b> <br/>
                        <input type="password" name="password" required/>
                    </label>
                    <a href="/forgotpassword">"Forgot password?"</a>
                </div>
                <input type="submit" value="Sign in"/>
            </form>

            <div class="create-account inputbox">
                <p>"New to Enzyme?"</p>
                <a href="/signup">"Create a profile"</a>
            </div>
        </div>
    }
}
