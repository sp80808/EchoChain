# EchoChain Backend API Services

## Overview
This service provides lightweight backend APIs for the EchoChain ecosystem, including user authentication, copyright checks, and integration with external APIs not suitable for direct blockchain or P2P interaction.

## Purpose
- Centralized services for authentication, copyright, and external integrations.
- Complements the decentralized blockchain and P2P systems.

## Architectural Design
- Microservices architecture (if multiple services).
- RESTful API design using Node.js/Express.
- Stateless services where possible.

## Core Technologies
- Node.js, Express, TypeScript
- JWT for authentication
- Integration with blockchain and P2P systems

## Dependencies
- External: Audio recognition APIs, email services
- Internal: EchoChain Blockchain, P2P File Sharing System

## API Specifications
- REST endpoints for user management, copyright checks, and metadata
- JWT-based authentication

## Data Models
- User, Session, CopyrightCheck, SampleMetadata

## Security & Compliance
- All secrets and environment variables are managed via `.env` (see .gitignore)
- No secrets or sensitive data are committed to the repository
- Follows best practices for authentication, authorization, and input validation

## Testing
- Unit and integration tests using Jest
- Test coverage goals: >90%

## .gitignore Rationale
- `node_modules/`, `dist/`, logs, and all secrets (`.env`) are ignored for security and compliance

## Getting Started
1. Install dependencies: `npm install`
2. Copy `.env.example` to `.env` and configure secrets
3. Start the service: `npm run start`

## Development Plan
- See main project documentation for detailed milestones and phases. 