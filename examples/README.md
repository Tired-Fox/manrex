# Examples

**SETUP**

1. Create a personal API client in your MangaDex account
    - MangaDex > Settings > API Clients > Create
2. Click on new API client and put the `CliendId` into an environment variable named `MANGADEX_CLIENT_ID`.
Also click `Get Secret` and then `Copy Secret` and put it in an environment variable named `MANGADEX_CLIENT_ID``.
3. When running the example for the first time, it will prompt for a username and password. These are the credentials for your MangaDex account. This is needed to be able to authenticate and get the initial auth token. As long as the token is not deleted, the examples should be able to refresh the token without re-authorizing.
