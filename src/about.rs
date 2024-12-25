use yew::prelude::*;
use crate::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <utils::TopBarDiv />
            <div class="bg-gradient-to-t from-emerald-950 to-green-900 h-screen w-full flex grid grid-cols-5 gap-4 mx-auto p-4">
                <div class="col-span-5 col-end-6 border-green-800 w-full p-4 rounded-2xl">
                    <h1 class="text-white font-bold text-5xl mx-auto text-center m-4">{"Hello!"}</h1>
                    <img class="float-right m-4 rounded-xl object-fit blur-sm" src="https://th.bing.com/th/id/OIP.2zQVH5kDo1q2CpeLccH5YAHaFi?rs=1&pid=ImgDetMain" />
                    <div class="space-y-4 m-6 text-xl text-white">
                        <p>{"I'm "} <a class="transition hover:border-b-2 hover:border-emerald-500 delay-100">{"Chaaruhaas Kandregula"}</a>{", but you can call me Chaas ;). I'm a junior at Lynbrook High School. I'm passionate about the physical sciences and I've started to dabble in programming "}<a class="transition hover:border-b-2 hover:border-amber-700 delay-100">{"(especially in Rust!)"}</a>{" , but I'll try anything that comes my way!"}</p>
                        <p>{"When I'm not working, you'll find me doing a whole host of things, such as playing chess/minecraft, doing cardistry, trying to teach myself guitar, going for a walk, or listening to music. I'm always looking for new adventures and experiences!"}</p>
                        <p>{"I believe that the best way to learn is to teach, so I'm always looking for opportunities to share my knowledge with, and learn from, others. I'm also a firm believer in the power of community and collaboration, and I'm always looking for ways to connect with others and work together to make the world a better place."}</p>
                        <p>{"Feel free to connect with me on discord (epictrombonekid) or drop me a line at chaaskandregula@gmail.com. I'd love to hear from you!"}</p>
                    </div>
                </div>
            </div>
        </>
    }
}

