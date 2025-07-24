# urlflatine

A web service that calculates a mubase multihash from a resource at a given URL

Developed for [EduBadges](https://edubadges.nl) for [SURF](https://www.surf.nl) by [Bèr Kessels](https://berk.es).

## Use-Case

In order to [add resource-integrity to a verifiable credential](https://www.w3.org/TR/vc-data-integrity/#resource-integrity), the issuance must add resource-integrity attributes to the credential.

E.g. an image, video or PDF, linked to from the credential, must be accompanied by a multibase multihash of the resource. 

At SURF, for EduBadges, we have several images in the credentials (OpenBadges v3, being Verifiable Credentials) that we issue, which are links to online files. We also allow any file to be used as [Evidence](https://www.w3.org/TR/vc-data-model-2.0/#evidence). For these, a multibase multihash must be calculated from the file before or during issuance.

In the spirit of our Architectural preference, we prefer small, simple, self-contained services that do one thing and do it well. Without dependency on other services.

Rust is our preferred language for services.

## Background

Microservice receives a URL, gets the resource at that URL, calculates a digest
of this resource, changes this into a multihash value, encodes this as multibase
and returns that.

* **multibase** is described in [the CID 1.0 spec](https://www.w3.org/TR/cid-1.0/#multibase-0), chapter '2.4 Multibase' and '4.3.1 The multibase Datatype'
* **multihash** is decribed in [the same spec](https://www.w3.org/TR/cid-1.0/#multihash) chapter '2.5 Multihash`.

## Web service

The service has only one component, which is the HTTP server. It has only one endpoint, which is the `/digest` endpoint. This endpoint accepts a URL as a query parameter and returns a multibase multihash of the resource at that URL.

## OpenAPI documentation

TODO: create an OpenAPI json or YAML describing the service.

## CLI

TODO: create a commandline runner that can be used to calculate a multibase multihash of a url.

## TODO

- [ ] create a commandline runner that can be used to calculate a multibase multihash of a url.
- [ ] allow providing hash type and algorithms on request
- [ ] add limits to size of the file
- [ ] add whitelist of allowed domains for URLs
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

### Install

TODO

### Run

TODO

### Test

TODO

### Release

TODO

### Deploy

TODO