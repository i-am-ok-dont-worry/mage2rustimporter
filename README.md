# magerustimporter
Data importer uses Magento REST API to synchronize Magento Database with ElasticSearch.

It synchronizes products, attributes, categories, taxrules, cmspages and cmsblocks.

### Synchronization
Synchronization is done via Magento REST API. In order to connect to Magento REST API
authorization parameters must be configured via `config.json` file:
```json
{
  ...
  "magento": {
    "url": "{MagentoRestApiUrl}",
    "consumerKey": "{ConsumerKey}",
    "consumerSecret": "{ConsumerSecret}",
    "accessToken": "{AccessToken}",
    "accessTokenSecret": "{AccessTokenSecret}"
   }
}
``` 

### Running importer
Importer uses `clap` to provide support for command arguments.
Run importer with following command:
```bash
./mage-rust-importer <adapter> -i <entities ids to import> -c <config file location>
```

### Configuration
In order to run importer configuration file must be supplied:
```json
{
  "redis": {
    "url": "{Redis url}"
  },
  "elasticsearch": {
    "url": "{ElasticSearch url}",
    "index": "{Index name prefix}"
  },
  "magento": {
    "url": "{MagentoRestApiUrl}",
    "consumerKey": "{ConsumerKey}",
    "consumerSecret": "{ConsumerSecret}",
    "accessToken": "{AccessToken}",
    "accessTokenSecret": "{AccessTokenSecret}"
  }
}
```
