use yew::prelude::*;
use crate::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
        <utils::TopBarDiv />
            <div class="bg-gradient-to-t from-green-950 to-green-900 h-screen w-full flex items-center justify-center grid grid-cols-3 gap-4 mx-auto p-4">
                <div class="text-white font-bold text-5xl mx-auto text-center m-4 col-span-3">{"Here are some projects I've worked on over the past couple of years!"}
                </div>
                <div class="col-span-3 grid grid-cols-3 gap-4 place-items-stretch h-full w-full">
                    <a href="https://github.com/EpicTromboneKid/scioly-bot" class="grid col-span-1 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"SciolyBot: A Discord Bot for Automatic Testing, built entirely in Rust!"}
                        <img src="https://cdn.discordapp.com/attachments/752638225008558192/1321359045826969620/sciolybot_1.png?ex=676cf302&is=676ba182&hm=9a5f5ba3a42d55eb9477ec7fa89eeb6e380c610e13a406c6b49cbb0618e9aee1&" alt="github" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="https://github.com/EpicTromboneKid/prevents_student_sleep" class="grid col-span-1 text-2xl font-semibold p-2 text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:scale-90 hover:shadow-inner focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"The Design and Construction of a Wearable to Prevent Students from Falling Asleep in Class"}
                        <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" alt="email" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="https://github.com/EpicTromboneKid/brainrot-terminator" class="grid col-span-1 p-4 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"A brainrot-terminator bot to keep discord channels free of it"}
                        <img src="https://cdn.discordapp.com/attachments/752638225008558192/1321360861838966825/brainrot.jpg?ex=676cf4b3&is=676ba333&hm=831c29dd3a05faeb2fbd8d8eb869fecc17f4882a6b7aae053b71fe11a8ce18c0&" alt="phone" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto" />
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