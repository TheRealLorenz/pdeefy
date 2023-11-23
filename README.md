# pdeefy

pdeefy (pronounced pee-dee-fy) is a simple service that leverages [chromium](https://github.com/chromium/chromium) to render HTML to PDF.

## Image

The project's main goal is to provide a lightweight image for html rendering. You could:

- Build the image yourself using the [Dockerfile](./Dockerfile).
- View the available builds on [docker-hub](https://hub.docker.com/repository/docker/thereallorenz/pdeefy/general).

## Compose file

The repository contains [compose.yaml](./compose.yaml) for quick deployment.

## Endpoint

It listens for POSTs on port 3000, path /api/generate

```jsonc
{
    "html": "<p>foo</p>",              // One of html or url is required
    "url": "https://foo.bar/baz",
    "options": {                       // Optional field
        // Refer to https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF.
        // for all available options
    }
}
```

### Return value

If the input renders successfully, a byte stream is returned.

Otherwise, the endpoint returns an error with the following shape:

```jsonc
{
    "message": "An error occurred"
}
```

