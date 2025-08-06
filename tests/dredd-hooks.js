const hooks = require('hooks');

// The hostname and port of our static file server inside the Docker network
const staticServerUrl = 'http://static-server:8000/example_asset.txt';
const nonExistingUrl = 'http://example.local:1337/';

// Hardcoded digest for our example file
// This value must match the SHA-256 digest of example_asset.txt in multibase (base64) format
const EXPECTED_DIGEST = 'mARISIMjc3XRZRHGI4RNvA7nnRu2UOQa6LMWGrHF81IwSkeGz';

hooks.before('/digest > Calculate content digest for a URL > 200 > application/json', (transaction) => {
    // Set URL to our example file on the static server
    const requestBody = JSON.parse(transaction.request.body);
    requestBody.url = staticServerUrl;
    transaction.request.body = JSON.stringify(requestBody);
});

// Verify the 200 response has the expected digest
hooks.after('/digest > Calculate content digest for a URL > 200 > application/json', (transaction, done) => {
  const responseBody = JSON.parse(transaction.real.body);
  if (responseBody.digest_multibase !== EXPECTED_DIGEST) {
        transaction.fail = `Digest does not match expected value: ${EXPECTED_DIGEST} !== ${responseBody.digest_multibase}`;
    }
    done();
});

hooks.before('/digest > Calculate content digest for a URL > 400 > application/json', (transaction) => {
    // Create an invalid request body without the required 'url' field
    transaction.request.body = JSON.stringify({
        not_url: "This field should cause a validation error"
    });
});

hooks.before('/digest > Calculate content digest for a URL > 500 > application/json', (transaction) => {
    // Set URL to a non-existent file on the static server
    const requestBody = JSON.parse(transaction.request.body);
    requestBody.url = nonExistingUrl;
    transaction.request.body = JSON.stringify(requestBody);
});
