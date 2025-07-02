# Echochain Blockchain GUI

This project is a React-based graphical user interface (GUI) for interacting with the Echochain blockchain.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

Make sure you have Node.js and npm installed.

*   Node.js (LTS recommended)
*   npm (comes with Node.js)

### Installation

1.  Clone the Echochain repository:
    ```bash
    git clone https://github.com/your-repo/echochain.git
    ```
2.  Navigate to the `Blockchain_GUI` directory:
    ```bash
    cd echochain/Blockchain/Blockchain_GUI
    ```
3.  Install the dependencies:
    ```bash
    npm install
    ```

### Running the Application

To run the application in development mode:

```bash
npm start
```

This will open the application in your browser at `http://localhost:3000`.

### Building for Production

To build the application for production:

```bash
npm run build
```

This command builds the app for production to the `build` folder. It correctly bundles React in production mode and optimizes the build for the best performance.

## Project Structure

- `src/components`: Reusable React components.
- `src/contexts`: React Context API for global state management.
- `src/services`: API interaction and other utility services.
- `src/App.js`: Main application component.
- `src/index.js`: Entry point of the React application.
