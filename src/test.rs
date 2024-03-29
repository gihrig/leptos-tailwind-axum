pub fn test_html() -> String {
    r#"
      <main>
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
          <div class="flex flex-row-reverse flex-wrap m-auto">
            <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
            "+"
            </button>
            <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-orange-600 border-blue-900 text-white">
            {value}
            </button>
            <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
            "-"
            </button>
        </div>
      </div>
      </main>
    "#
}
