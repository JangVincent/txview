# txview

Txview is a cli-based tool for viewing transactions on the ethereum compatible blockchains using [Infura](https://infura.io/)

### Current Version : 0.1.1

## Installation

```bash
brew tap VincentJang/tap
brew install txview
```

## Usage

1. Create a infura account for use infura api key. [Infura](https://infura.io/)

2. Make a file in `$HOME/.config/txview/config`
```bash
cd $HOME/.config/
mkdir txview && cd txview
touch config
```

3. Write your infura api key in config file purely (not necessary any other string).
```bash
your_infura_api_key
```
like,
```bash
1234567890abcdefg
```

4. Run txview
```bash
# txview [network] [tx-hash]
txview eth-goerli 0x-tx-hash
```

## Support chain
```bash
txview --help // You can find support chain
```


## Disclaimer
### Infura Rate Limit
This program is affected by the rate limit applied to your API key.  
See [Infura pricing](https://www.infura.io/pricing)

### Infura API key
This program does not use your API key anywhere except to use an infura request.  
This program/developer is not responsible for any issues related to API keys.
