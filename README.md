# krusty
Add-on for weathica that exports your investment portfolio into a local Postgres instance. Written in Rust 


## Motivation
I am using [Wealthsimple](https://www.wealthsimple.com/en-ca) as my Canadian broker. The special thing about Wealthsimple is that it doesn't charge any fees when you are buying stocks from Toronto Exchange. However, the web interface is limited and doesn't provide a comprehensive statistics(stocks you are holding divided by sector or how many dividends you received in last month). To eliminate this problem you can use [Wealthica](https://wealthica.com/) which is a free aggregator website that can show your portfolio from different brokers. Wealthica gives a public API to gather your data.

## Krusty
Krusty is a add-on + backend written in Rust. The add-on is injected into wealthica and then sends all your data into a Rust based Backend.

## Instructions
1. Integrate Wealthica with your Wealthsimple account
2. Add [Wealthica Developer widget](https://github.com/wealthica/wealthica.js)
3. Configure it to use an url that points to the [frontend page](https://github.com/strogiyotec/krusty/blob/master/frontend/index.html) (Your server that returns this html page has to be configured with https , self signed certificate is good enough)
4. Press Transactions button to send your portfolio into a Rust backend

## How to use 
The frontend page provides two buttons, one to save all your stocks into the database and another one to get all your dividends

## TODO
1. [X] Implement dividends button
2. [ ] Show YAHOO stats for each individual ticker


