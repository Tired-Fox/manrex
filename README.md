# ManRex

A Rust based [MangaDex](https://mangadex.org/) client. This is a light, yet powerful, client to interact with the MangaDex API.

## Disclaimer

ManRex is not affiliated with [MangaDex](https://mangadex.org/).

This project requests that all users adhere to MangaDex's [Acceptable Usage Policy](https://api.mangadex.org/docs/#acceptable-usage-policy).

1. You **MUST** credit MangaDex
2. You **MUST** credit scanlation groups (and honor their content removal requests) if you allow reading chapters
3. You **CANNOT** run ads or paid services on your website and/or apps

## Features

- Full `MangaDex` public API implementation (within reason) found [here](https://api.mangadex.org/docs/swagger.html#/) with it's docs found [here](https://api.mangadex.org/docs/)
- Opt-in client side rate limiting to avoid hitting the API's automatic rate limiting
- Caching of endpoints like the `/at-home` api that gives limited lifetime urls and endpoints. ([ref](https://api.mangadex.org/docs/04-chapter/retrieving-chapter/#howto))
- Automatic image success/failure reporting ([ref](https://api.mangadex.org/docs/04-chapter/retrieving-chapter/#mangadexhome-load-successes-failures-and-retries))
- Generic enough client that can be used in web, application, tui, cli, and other environments.

## Rate Limiting

MangaDex implements rate limiting of around 5 http requests per minute. On top of this, it also implements a rate limit for each endpoint. When a rate limit is reached a HTTP 429 response is returned until a minimum amount of time for the endpoint rate
limit or HTTP rate limit to reset. If there is repeat and excesive offenses, the IP that is making the requests will get a temporary ban.

To help the consumer to avoid this, this client has rate limiting and caching solutions builtin that are opt-in by the consumer. This will keep the rate limits, along with contributers to the rate limit, to a minimum allowing the consumer to better work
around them without hitting a temporary ban.

## Sources

- [rust-lang](https://www.rust-lang.org/)
- [Hypr](https://hyper.rs/)
- [MangaDex](https://api.mangadex.org/docs/#acceptable-usage-policy)
- [MangaDex Redoc](https://api.mangadex.org/docs/redoc.html#tag/Manga/operation/get-manga-random)
- [MangaDex Swagger](https://api.mangadex.org/docs/swagger.html#/)
- [MangaDex Endpoint Rate Limiting](https://api.mangadex.org/docs/2-limitations/#endpoint-specific-rate-limits)

## Other Projects

- https://docs.rs/mangadex/latest/mangadex/
- https://docs.rs/mangadex-api-rust/latest/mangadex_api/

Huge credit to [MangaDex.org](https://mangadex.org/) for being a non profit that creates and maintains the `api.mangadex.org` and `api.mangadex.dev` APIs.
