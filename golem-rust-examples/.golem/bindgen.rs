
wit_bindgen::generate!({
    inline: "
package golem:test;
interface api {
   record person {
      name: string,
      address: address
   }

   record address {
      street: option<string>,
      city: option<result<string, string>>,
      state: string,
      zip: string,
      color: color
   }

   enum color {
      red,
      green,
      blue,
      blue-green
   }

   variant variant-test {
      v1(string),
      v2(list<string>)
   }

   record bidder-id {
      bidder-id: result<color, string>,
      verified: bool
   }

   create-bidder: func(full-name: string, address: list<string>, age: option<u16>) -> bidder-id;

   get-address: func() -> address;


}

world test-worker {
   export api;
}
",
});
