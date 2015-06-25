rustybitmaps
=========

RustyBitmaps is a GC-free compressed bitmap microservice written in Mozilla's programming language <a href="http://www.rust-lang.org/">Rust</a>. The program starts a web server with a simple JSON-RPC API and is backed by an open source implementation of the <a href="http://roaringbitmap.org/">Roaring bitmaps</a> compression for bitset data. The server is multi-threaded with memory read/write synchronization on each bitmap (requests can concurrently access different bitmaps). 

<h2>Building and running</h2>
Install v1.0 of <a href="http://www.rust-lang.org/">Rust</a>. Clone this repository. Run "cargo --release" in the root folder. 

The compilation and running of the included web server may require that you install binary and developer packages of OpenSSL (see build instructions of the <a href="https://github.com/sfackler/rust-openssl">OpenSSL crate</a>).

To start the server with console log level set to info, go to the target/release folder and do<br/>
<b>$ RUST_LOG=info ./rustybitmaps</b>

<h2>JSON-RPC API</h2>
Server starts accepting connections on 127.0.0.1:8081 and receives POST requests on the path /. All parameters should be passed in as a list of strings, ["", ""..], even though they are in practice numbers.
The server supports the following methods

method: <b>create_new</b>, params: []<br/>
Returns a string with the ID of the new bitmap.

method: <b>insert_item</b>, params: ["item index", "bitmap ID 1", "bitmap ID 2"...]<br/>
Inserts the item with numerical index "item_index" into the bitmaps with the specified IDs.

method: <b>contains_item</b>, params: ["item index", "bitmap ID 1", "bitmap ID 2"...]<br/>
Returns "true" if the item index had been previously inserted in ALL of the bitmaps with the specified indices.

<h2>Limitations</h2>
<ul>
<li>No persistence, everything gets cleared when the server is restarted</li>
<li>Not tested/tuned for either quality or performance</li>
<li>First experiment with language</li>
</ul>
