
var ffi  = require ('ffi');
var path = require ('path');


var lib = ffi.Library (
	path.join (__dirname,
			  './target/debug/libfibo'), {

	fibo: ['int', ['int']]

});


var n = lib.fibo (20);

console.log (n);
