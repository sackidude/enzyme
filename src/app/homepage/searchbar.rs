use leptos::*;

#[component]
pub fn SearchBar() -> impl IntoView {
    view! {
        <div class="searchbar">
            // TODO!: Fix this temporary solution
            <script>
                // Courtesy of ChatGPT
                "function redirect() {
                    // Get the value of the input field
                    var name = document.getElementById('account-name').value;
                
                    // Construct the URL with the name parameter
                    var url = '/account/' + name;
                
                    // Redirect to the constructed URL
                    window.location.href = url;
                
                    // Prevent the default form submission
                    return false;
                }"
            </script>
            <form onsubmit="return redirect()">
                <img src="search-glass.svg" alt=""/>
                <input
                    type="text"
                    name="account"
                    placeholder="Search for an account"
                    id="account-name"
                />
            </form>
        </div>
    }
}