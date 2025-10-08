# MongoDB Atlas Local Library (Node.js)

![CI](https://github.com/mongodb-js/atlas-local-lib-js/workflows/CI/badge.svg)

A Node.js library for managing MongoDB Atlas Local deployments using Docker. This library provides a high-level JavaScript/TypeScript interface to interact with MongoDB Atlas Local deployments, making it easy to develop and test applications against a local MongoDB Atlas environment.

## Overview

MongoDB Atlas Local Library simplifies the process of managing MongoDB Atlas Local deployments by providing a Node.js-native interface that abstracts away the complexity of Docker container management. Whether you're developing applications that will eventually run against MongoDB Atlas or testing Atlas-specific features locally, this library provides the tools you need.

## Features

- **Docker Integration**: Seamlessly manages MongoDB Atlas Local deployments through Docker
- **TypeScript Support**: Built with TypeScript with full type definitions
- **Cross-Platform**: Works on macOS, Linux, and Windows
- **Simple Setup**: Easy to integrate into existing Node.js projects
- **Development Ready**: Perfect for local development and testing workflows

## Installation

```bash
npm install @mongodb-js-preview/atlas-local
# or
yarn add @mongodb-js-preview/atlas-local
```

## Prerequisites

Before using this library, make sure you have:

- **Docker**: Docker must be installed and running on your system
- **Node.js**: Node.js 16+ with full Node-API support

## Quick Start

Here's a simple example to get you started:

```typescript
const client = await Client.connect();
const deployments = await client.listDeployments();
console.log(deployments);
```

## Development

### Build

```bash
yarn build
```

### Test

```bash
yarn test
```

### Benchmark

```bash
yarn bench
```

## API Documentation

The complete API documentation is available in the generated TypeScript definitions. Key components include:

- **`Client`**: The main entry point for managing MongoDB Atlas Local deployments
- **`Deployment`**: Interface representing a MongoDB Atlas Local deployment
- **Error Types**: Comprehensive error handling for Docker and Atlas operations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## License

This project is licensed under the Apache License, Version 2.0 (see `LICENSE`). It also makes use of third-party libraries, which are distributed under their own respective licenses (see `LICENSE-3RD-PARTY.txt`).

## Related Projects

- [MongoDB Atlas](https://www.mongodb.com/atlas) - MongoDB's cloud database service
- [MongoDB Atlas Local](https://www.mongodb.com/docs/atlas/cli/current/atlas-cli-deploy-local/) - Local development environment for MongoDB Atlas
