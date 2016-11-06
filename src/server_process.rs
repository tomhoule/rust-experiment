impl ServerProcess {
    fn start() {
    }
}

/**
 * The server process will be listening on stdin and stdout, we need to start it ourselves.
 */
trait ServerProcess {
    fn start();
}
