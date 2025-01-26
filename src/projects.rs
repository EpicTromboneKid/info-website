use crate::*;
use yew::prelude::*;

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
                        <img src="https://cloud-kwoda6ji7-hack-club-bot.vercel.app/1sciolybot__1_.png" alt="github" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="https://github.com/EpicTromboneKid/prevents_student_sleep" class="grid col-span-1 text-2xl font-semibold p-2 text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:scale-90 hover:shadow-inner focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"The Design and Construction of a Wearable to Prevent Students from Falling Asleep in Class"}
                        <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" alt="email" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto"/>
                    </a>
                    <a href="https://github.com/EpicTromboneKid/brainrot-terminator" class="grid col-span-1 p-4 text-2xl font-semibold text-white mx-auto items-center text-center object-center rounded-lg border border-green-800 shadow-2xl h-full w-full flex justify-center overflow-hidden hover:shadow-inner hover:scale-90 focus:outline-none focus:ring-2 focus:ring-green-400">
                        {"A brainrot-terminator bot to keep discord channels free of it"}
                        <img src="https://cloud-kwoda6ji7-hack-club-bot.vercel.app/0brainrot.jpg" alt="phone" class="rounded-3xl col-span-1 items-center w-8/12 mx-auto" />
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
