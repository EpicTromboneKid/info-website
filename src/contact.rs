use yew::prelude::*;
use crate::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <utils::TopBarDiv />
            <div class="bg-gradient-to-t from-green-950 to-green-900 h-screen w-full flex items-center justify-center grid grid-cols-3 gap-4 mx-auto p-4">
                <div class="text-white font-bold text-5xl mx-auto text-center m-4 col-span-3">{"Feel free to contact me!"}
                <h1 class="text-white font-bold text-2xl mx-auto col-span-3">{"(Most) of my social media usernames are EpicTromboneKid!"}</h1>
                </div>
                <div class="col-span-3 grid grid-cols-3 gap-4 place-items-stretch h-full w-full">
                    <a href="https://www.github.com/EpicTromboneKid" class="grid col-span-1 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"Github"}
                        <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" alt="github" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="mailto:chaaskandregula@gmail.com" class="grid col-span-1 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:scale-90 hover:shadow-inner focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"Email: chaaskandregula@gmail.com"}
                        <img src="https://cdn4.iconfinder.com/data/icons/green-shopper/1049/email.png" alt="email" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="" class="grid col-span-1 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"Discord: epictrombonekid"} <br />{"(this link does not lead anywhere)"}
                        <img src="https://static.vecteezy.com/system/resources/previews/018/930/718/original/discord-logo-discord-icon-transparent-free-png.png" alt="phone" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto" />
                    </a>
                    // <a href="[Your Address Link]" class="grid col-span-1 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                    //     {"Links: [Your Address]"}
                    //     <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" alt="links" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    // </a>
                </div> 
            </div>
        </>
    }
}