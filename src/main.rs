#[cfg(target_arch = "wasm32")]
use gloo_console::log;
#[cfg(target_arch = "wasm32")]
use serde_json::Value;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{EventTarget, HtmlInputElement};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[function_component]
fn MyComponent() -> Html {
    let filter_value = use_state(|| r#">/#pick(>/..priority/#len as 'total_prios', >/entry/values/#filter('name' == 'gearbox') as 'kind')"#.to_string());
    let input_value_handle = use_state(|| r#"{"entry": {"values": [{"name": "gearbox", "priority": 10}, {"name": "steam", "priority": 2}]}}"#.to_string());
    let pretty_value = use_state(Vec::<String>::default);
    let pretty_input_value = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let filter_field = filter_value.clone();
    let on_dangerous_change = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        let value = target.unchecked_into::<HtmlInputElement>().value();
        filter_field.set(value.clone());
    });

    let filter_field = filter_value.clone();
    let pretty_field = pretty_value.clone();
    let pretty_input = pretty_input_value.clone();
    let input_handle = (*input_value_handle).clone();
    let onclick = Callback::from(move |_| {
        let j: Value = serde_json::from_str(&input_handle).unwrap();
        pretty_input.set(serde_json::to_string_pretty::<Value>(&j).unwrap());

        let mut result: Vec<String> = vec![];
        let pr = jetro::context::Path::collect(j, &*filter_field);
        if pr.is_err() {
            pretty_field.set(vec![pr.err().unwrap().to_string()]);
            return;
        }
        for v in &pr.unwrap().0 {
            log!(serde_json::to_string_pretty::<Value>(v).unwrap());
            result.push(serde_json::to_string_pretty::<Value>(v).unwrap());
        }
        pretty_field.set(result);
    });

    let filter_value_front = (*filter_value).clone();

    #[rustfmt::skip(html)]
    html! {
    <>
    <nav class="navbar is-transparent">
	<div class="navbar-brand">
	    <div class="navbar-burger" data-target="navbarExampleTransparentExample">
		<span></span>
		<span></span>
		<span></span>
	    </div>
	</div>

	<div id="navbarExampleTransparentExample" class="navbar-menu">
	    <div class="navbar-start">
		<a class="navbar-item" href="https://jetro.io">
		    {"Home"}
		</a>
		<div class="navbar-item has-dropdown is-hoverable">
		    <a class="navbar-link" href="https://jetro.io">
			{"Actions"}
		    </a>
		    <div class="navbar-dropdown is-boxed">
			<a class="navbar-item" href="https://github.com/mitghi/jetro/blob/main/README.md">
			    {"Examples"}
			</a>
		    </div>
		</div>
	    </div>

	    <div class="navbar-end">
		<div class="navbar-item">
		    <div class="field is-grouped">
			<p class="control">
			    <a class="bd-tw-button button" target="_blank" href="https://github.com/mitghi/jetro">
				<i class="fab"></i>
				<span>
				    {"GitHub"}
				</span>
			    </a>
			</p>
		    </div>
		</div>
	    </div>
	</div>
    </nav>
    <div class="container hero is-fullheight">
      <div class="content is-normal">
	<h1 class="title">{"⚙️ Jetro"}</h1>
	<h6 class="subtitle">{"Webassembly frontend for Jetro, JSON transformer and search query"}</h6>
        <h2>{"REPL"}</h2>
      </div>
      <div>
        <div>
          <label for="cautious-input">
	    { "JSON" }
	    <input onchange={on_cautious_change}
	      class="input is-primary" type="text" placeholder="input"
	      id="cautious-input"
	      type="text"
	      value={input_value.clone()}
	      />
	    </label>
          </div>
          <div>
	    <label for="dangerous-input">
	      { "search:" }
	      <input onchange={on_dangerous_change}
		class="input is-primary" type="text" placeholder="search query"
		id="dangerous-input"
		type="text"
		value={filter_value_front.clone()}
		/>
	      </label>
	    </div>
	    <div class="buttons">
	      <button {onclick} class="button is-primary">{ "Execute" }</button>
	    </div>

          </div>

          <div class="columns">
	    <div class="column is-half" >
	      <h4>{"input ( formatted )"}</h4>
	      <pre>{&*pretty_input_value}</pre>
	    </div>
	    <div class="column is-half">
	      <h4>{"output ( formatted )"}</h4>
	      {
	      pretty_value.iter().map(|v| {
	      html!{<pre>{format!("{}", v)}</pre>}
	      }).collect::<Html>()
	      }
	    </div>
          </div>

	  <footer class="has-text-centered is-flex-align-items-flex-end mt-auto">
	    <span>
	      <strong>{"Jetro "}</strong> {"by"} <a href="https://github.com/mitghi">{" Mike Taghavi"}</a>{"."}
	    </span>
	    <br />
	    {"The source code is licensed"}
	    <a href="http://opensource.org/licenses/mit-license.php">{" MIT"}</a>{"."} {" The website content
	    is licensed "}<a href="http://creativecommons.org/licenses/by-nc-sa/4.0/"> {" CC BY NC SA 4.0"}</a>{"."}
	  </footer>
        </div>
        </>
    }
}

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    html! {
        <><MyComponent/></>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<App>::new().render();
}


#[cfg(not(target_arch = "wasm32"))]
use actix_web::{App, HttpServer};
#[cfg(not(target_arch = "wasm32"))]
use envconfig::Envconfig;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "JETRO_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "JETRO_PORT", default = "8080")]
    pub port: i16,
}

#[cfg(not(target_arch = "wasm32"))]
impl Config {
    fn addr(&self) -> String {
	format!("{}:{}", &self.host, &self.port)
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_health_status() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body("OK")
}

#[cfg(not(target_arch = "wasm32"))]
use actix_web_static_files::ResourceFiles;

#[cfg(not(target_arch = "wasm32"))]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));


#[cfg(not(target_arch = "wasm32"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init_from_env().expect("unable to parse config");
    let addr = config.addr();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));    

    log::info!("starting HTTP server at http://{}", &addr);

    HttpServer::new(|| {
        App::new()
            .route("/health", actix_web::web::get().to(get_health_status))
            .service(ResourceFiles::new("/", generate()))
    })
    .bind(addr)?
    .run()
    .await
}
