import * as multipass from "../pkg/index.js"

const authButton = document.getElementById("authorize-twitter");
authButton.addEventListener("click", event => {
	buttonPress()
})

async function buttonPress() {
	const id = await multipass.get_user_profile();
}
