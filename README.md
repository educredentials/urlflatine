# urlflatine

A web service that calculates a mubase multihash from a resource at a given URL

Developed for [EduBadges](https://edubadges.nl) for [SURF](https://www.surf.nl) by [Bèr Kessels](https://berk.es).

## Why?

Verifiable Credentials can reference external resources. E.g. a link to a video-file for *evidence* or a link to an image for logo or your passport photo. To ensure this file was not tampered with during verification, we need to calculate a hash that enables **Resource Integrity** in **Verifiable Credentials**.

Calculating this hash is complex, requires a broad range of standards, and  is easy to get wrong with regards to e.g. security.

We therefore developed a central service that calculates this hash.

## Usage

```
curl -X POST -H "Content-Type: application/json" -d '{"url": "https://example.com/videos/never_gonna_give_you_up.mp4"}' https://urlflatine.example:8080/digest
```

## Background

In order to [add resource-integrity to a verifiable credential](https://www.w3.org/TR/vc-data-integrity/#resource-integrity), the issuance must add resource-integrity attributes to the credential.

E.g. an image, video or PDF, linked to from the credential, must be accompanied by a multibase multihash of the resource. 

At SURF, for EduBadges, we have several images in the credentials (OpenBadges v3, being Verifiable Credentials) that we issue, which are links to online files. We also allow any file to be used as [Evidence](https://www.w3.org/TR/vc-data-model-2.0/#evidence). For these, a multibase multihash must be calculated from the file before or during issuance.

In the spirit of our Architectural preference, we prefer small, simple, self-contained services that do one thing and do it well. Without dependency on other services.

Rust is our preferred language for services.

Microservice receives a URL, gets the resource at that URL, calculates a digest
of this resource, changes this into a multihash value, encodes this as multibase
and returns that.

* **multibase** is described in [the CID 1.0 spec](https://www.w3.org/TR/cid-1.0/#multibase-0), chapter '2.4 Multibase' and '4.3.1 The multibase Datatype'
* **multihash** is decribed in [the same spec](https://www.w3.org/TR/cid-1.0/#multihash) chapter '2.5 Multihash`.

## Web service

The service has only one component, which is the HTTP server. It has only one endpoint, which is the `/digest` endpoint. This endpoint accepts a URL as a query parameter and returns a multibase multihash of the resource at that URL.

## CLI

TODO: create a commandline runner that can be used to calculate a multibase multihash of a url.

## TODO

- [ ] make the API "async": return a 201 with a location header pointing to the created resource instead of a 200 with the resource.
- [ ] create a commandline runner that can be used to calculate a multibase multihash of a url.
- [ ] allow providing hash type and algorithms on request. e.g. `{ "hash-function": "sha3-512" }`.
- [ ] allow providing encoding type and request. e.g. `{ "encoding-function": "base58btc" }`.
- [ ] add limits to size of the file. E.g. configurable with an ENV var on runtime.
- [ ] add whitelist of allowed domains for URLs. E.g. configurable with an ENV var on runtime.
- [ ] add caching: persist the url (and other request params) and its cached multibase multihash in KV storage and use this to avoid recalculating the multihash.
- [ ] use the caching to send proper caching headers
- [ ] add full CRUD for a "url+request-params" resource:
  - [x] POST: create a new resource
  - [ ] GET: read a resource, return the hash if already created - allows looking up previously made hashes
  - [ ] PUT: update a resource. Replace the existing hash with a new one.
  - [ ] DELETE: delete a resource. Delete the resource and its associated multibase multihash.

## Quickstart

1. Checkout the repository: `git clone https://github.com/yourusername/yourrepository.git`
2. Build the Docker image: `docker build -t urlflatine .`
3. Run the Docker container: `docker run -p 8080:8080 urlflatine`

4. Test the service by sending a request:  `curl -X POST http://localhost:8080/digest?url=https://example.com`

## API Documentation

See [openapi.yaml](openapi.yaml) file for the API specification.

### Install

TODO

### Run

Running in Docker.

Either build the image, or pull from Surf Container Registry:
```
docker build -t urlflatine .
```

Pull. At moment of writing, this image is still private so access is restricted to authorized users.
```
docker pull cr.surf.nl/edubadges-edubadges/edubadges/urlflatine:latest
```

Then run. Provide two environment variables:

* `LISTEN_HOST` - The host to listen on. Defaults to `127.0.0.1`.
* `LISTEN_PORT` - The port to listen on. Defaults to `8080`.

```
docker run -p 8080:8080 -e LISTEN_HOST=0.0.0.0 -e LISTEN_PORT=8080 urlflatine
```

### Test

TODO

### Release

TODO

### Deploy

TODO