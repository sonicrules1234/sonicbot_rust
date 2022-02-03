#Installation from crates.io
'''
cargo install sonicbot
'''

Run sonicbot_rust once to generate a blank config file named conf.json.  Then fill out the config file and run sonicbot_rust again.  The location of the config file on android is sdcard/Android/media/rust.sonicbot/conf.json

Note: If you are upgrading from version 0.1.2, you need to add the hostlabel key and a value to go with it to your conf.json.


Note 2: If you are upgrading from version 0.1.5, in your conf.json file, you need to add a "[" (without quotes) at the beginning and a "]" (without quotes) at the end to make it a json list/array because 0.1.6 supports being connected to multiple networks at the same time.


Note 3: If you are upgrading from version 0.1.7, you need to make your conf.json look like conf.json.dist.  You will also have to delete the data storage files for your networks.  They all start with sonicbotdata_

