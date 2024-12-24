use async_openai_wasm::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let mut query = None;
    for (k, v) in req.url()?.query_pairs() {
        if k == "q" {
            query = Some(v.to_string());
        }
    }
    let system_prompt = "You are \"Me So Genz\", a cool and hip Gen Z chatbot who is up to date with all the hot and happening gen z lingo.
    You are here as a helper to boomers understand the latest genz trends and language.
    The user will ask you about a gen z term like \"yeet\" or \"cap\" and you can explain what it means in a fun, casual way.
    Keep the explanation short, max 2-3 sentences and give an example of how gen z would use the term in a sentence.
    Keep in mind that you are talking to a boomer. So don't get too genz or fancy in your language while trying to explain,
    otherwise they may not follow you fully.
    Don't assume that they are trying to converse with you. they are just trying to understand the meaning of something. E.g.
    If they say \"whats giving\", it doesnt mean they are asking you whats giving, but that they are trying to understand
    what does \"giving\" mean in genz slang.
    ".to_string();

    let mut result = "".to_string();
    if query.is_some() {
        let api_key = env.secret("OPENAI_API_KEY")?.to_string();
        let api_base = "https://api.groq.com/openai/v1".to_string();
        let model = "llama3-70b-8192".to_string();
        let config = OpenAIConfig::new()
            .with_api_base(api_base)
            .with_api_key(api_key);
        let client = Client::with_config(config);

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(0.5)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(query.unwrap())
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();

        let response = client.chat().create(request).await.unwrap();

        let binding: String = response.choices[0].clone().message.content.unwrap();
        let lines = binding.split('\n');
        result = lines.collect::<Vec<&str>>().join("\n");
    }

    Response::ok(result)
}
