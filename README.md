# Rust-Vue Skeleton

Really simple web application to show off how to use [Rust](https://rust-lang.org/) and [Vue](https://vuejs.org/) together.

## Application structure

### Backend

In `src/main.rs` is the entirety of the backend. It uses [Axum](https://docs.rs/axum/latest/axum/) to route HTTP
requests to the `/counter` endpoint. Making a GET request to `/counter` returns
the current value in the counter, and making a POST request to `/counter`
increments the counter. That router can be seen here:

```rs
let router = axum::Router::new()
    .route("/counter", get(read_counter))
    .route("/counter", post(inc_counter))
    .with_state(app_state)
    .layer(CorsLayer::permissive());
```

The two endpoint functions are very simple. They use an app state to store the
value of a counter, and access it through `read` and `inc` functions:

```rs
async fn read_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    Json(ReadCounter {
        counter: app_state.read().await,
    })
}

async fn inc_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state.inc().await;
}
```

### Frontend

The Vue app is in `frontend/src/App.vue`. It uses `read_counter` and `increment`
to read and modify the value of the counter on the backend. This is done by
making requests to the `/counter` endpoint. The value of the counter is stored
as a global variable.

```js
const counter = ref<{ counter: number }>({ counter: 0 })

function read_counter() {
  fetch("http://localhost:3000/counter")
    .then(response => response.json())
    .then((data: any) => {
      if (data && typeof data.counter === 'number') counter.value = data
    })
    .catch(() => {})
}

function increment() {
  fetch("http://localhost:3000/counter", {
    method: "POST",
    headers: { "Content-Type": "application/json" }
  }).catch(() => {})
}
```

The counter is refreshed every second:

```js
onMounted(() => {
  read_counter()
  pollTimer = window.setInterval(read_counter, 1000)
})
```

And is displayed on the web page along with a button to increment the counter:

```html
<p>The counter is at: {{ counter.counter }}</p>
<input type="button" @click="increment" value="button" />
```

## Running

Run the following [Docker](https://docs.docker.com/get-started/get-docker/) command:

```
docker compose up --build -d
```

and then navigate to http://localhost:5173/ in your browser.

To shutdown the containers and delete any associated volumes, run:

```
docker compose down -v
```

