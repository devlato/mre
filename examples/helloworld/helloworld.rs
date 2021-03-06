fn main() {
    let mre = mre::MRE(
        // Create a zeromq context that MRE will use to talk to Mongrel2.
        match zmq::init(1) {
            Ok(ctx) => ctx,
            Err(e) => fail e.to_str(),
        },

        // A UUID for this Mongrel2 backend.
        Some(~"E4B7CE14-E7F7-43EE-A3E6-DB7B0A0C106F"),

        // The addresses to receive requests from.
        ~[~"tcp://127.0.0.1:9996"],

        // The addresses to send responses to.
        ~[~"tcp://127.0.0.1:9997"],

        // Create our middleware, which preproceses requests and
        // responses. For now we'll just use the logger.
        //~[mre::middleware::logger(io::stdout())],

        // A function to create per-request data. This can be used by
        // middleware like middleware::session to automatically look
        // up the current user and session data in the database. We don't
        // need it for this example, so just return a unit value.
        || ()
    );

    // Route our responses.
    do mre.get(~"^/$") |_req, rep, _m| {
        rep.reply_html(200u,
            ~"<html>\n\
              <body>\n\
              <h1>Hello world!</h1>\n\
              </body>\n\
              </html>")
    }

    // Finally, start the MRE event loop.
    mre.run();
}
