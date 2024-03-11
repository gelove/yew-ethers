//网络数据源 https://github.com/ethereum-lists/chains/tree/master/_data/icons
window.__CHAINS__ = {
  1: {
    i18nId: "eth",
    icon: "/asset/image/chains/eth.svg",
    isEVM: true,
    blockGasLimit: 30000000,
    chainId: "1",
    chainName: "Ethereum Mainnet",
    nativeCurrency: {
      name: "Ethereum",
      symbol: "ETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://rpc.ankr.com/eth", "https://main-rpc.linkpool.io", "https://main-light.eth.linkpool.io", "https://rpc.ankr.com/eth", "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://cn.etherscan.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.etherscan.io",
        apikey: "QA7WHR6U3N9Y3CCGGIKCT1U9PKI9D73CMR",
      },
    ],

    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  56: {
    i18nId: "binance",
    icon: "/asset/image/chains/binance.svg",
    chainId: "56",
    isEVM: true,
    blockGasLimit: 70000000,
    chainName: "Binance Smart Chain Mainnet",
    nativeCurrency: {
      name: "Binance Smart Chain Native Token",
      symbol: "BNB", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: [
      "https://bsc-dataseed1.binance.org",
      "https://bsc-dataseed2.binance.org",
      "https://bsc-dataseed3.binance.org",
      "https://bsc-dataseed4.binance.org",
      "https://bsc-dataseed1.defibit.io",
      "https://bsc-dataseed2.defibit.io",
      "https://bsc-dataseed3.defibit.io",
      "https://bsc-dataseed4.defibit.io",
      "https://bsc-dataseed1.ninicoin.io",
      "https://bsc-dataseed2.ninicoin.io",
      "https://bsc-dataseed3.ninicoin.io",
      "https://bsc-dataseed4.ninicoin.io",
      // "wss://bsc-ws-node.nariox.org",
    ],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "https://api.bscscan.com",
        apikey: "2M6BSP7KM897H26KBTUUQZRW19921MIFWE",
      },
    ],
    blockExplorerUrls: ["https://bscscan.com"],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  43114: {
    i18nId: "avax",
    icon: "/asset/image/chains/avax.svg",
    chainId: "43114",
    isEVM: true,
    blockGasLimit: 8000000,
    chainName: "Avalanche Mainnet",
    nativeCurrency: {
      name: "Avalanche",
      symbol: "AVAX", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://api.avax.network/ext/bc/C/rpc"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://snowtrace.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.snowtrace.io",
        apikey: "U21VSUEMK55FZTKPPQQUVRAUEUG6H8E8QG",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy", "EIP-1559"],
  },
  128: {
    i18nId: "heco",
    icon: "/asset/image/chains/heco.svg",
    chainId: "128",
    isEVM: true,
    blockGasLimit: 40000000,
    chainName: "Huobi ECO Chain Mainnet",
    nativeCurrency: {
      name: "Huobi ECO Chain",
      symbol: "HT",
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://http-mainnet.hecochain.com", "https://http-mainnet-node.huobichain.com", "https://pub001.hg.network/rpc"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://hecoinfo.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.hecoinfo.com",
        apikey: "HQTZDKIIR2A5I7UQVZT9FYQBRX97XUIE4S",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy", "EIP-1559"],
  },

  // https://app.saber.so/
  //   https://docs.solflare.com/technical/connecting-your-solana-dapp-with-solflare
  //   https://docs.phantom.app/integrating/detecting-the-provider
  solana: {
    i18nId: "solana",
    icon: "/asset/image/chains/solana.svg",
    isEVM: false,
    blockGasLimit: 40000000,
    chainId: "solana", // solana chain id
    chainName: "Solana Mainnet",
    nativeCurrency: {
      name: "Solana Mainnet",
      symbol: "SOL",
      decimals: 9,
    },
    networkType: 1,
    rpcUrls: ["https://api.mainnet-beta.solana.com", "https://mainnet-beta.solflare.network", "https://solana-mainnet.phantom.tech"],
    blockExplorerType: "solscan",
    blockExplorerUrls: ["https://explorer.solana.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://public-api.solscan.io",
        apikey: "",
      },
    ],

    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/extrinsic/${address}`,
    supportedWallets: ["phantom", "SolflareExtension"],
    txnType: ["Legacy"],
  },

  //   https://wiki.acala.network/integrate/integration/networks
  // https://polkadot.js.org/docs/api/start/create
  // https://polkadot.js.org/docs/extension/usage
  acala: {
    i18nId: "acala",
    icon: "/asset/image/chains/acala.jpg",
    isEVM: false,
    blockGasLimit: 40000000,
    chainId: "acala",
    chainName: "Acala Mainnet",
    nativeCurrency: {
      name: "Acala Mainnet",
      symbol: "ACA",
      decimals: 12,
    },
    networkType: 1,
    rpcUrls: ["https://acala-polkadot.api.onfinality.io/public-rpc"],
    blockExplorerType: "subscan",
    blockExplorerUrls: ["https://acala.subscan.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://acala.api.subscan.io",
        apikey: "",
      },
    ],

    txLinkFormat: (url, txid) => `${url}/extrinsic/${txid}`,
    accountFormat: (url, address) => `${url}/extrinsic/${address}`,
    supportedWallets: ["polkadot.js"],
    txnType: ["Legacy"],
  },

  //   https://internal-api.elrond.com
  // https://maiar.exchange/swap
  elrond: {
    i18nId: "elrond",
    icon: "/asset/image/chains/elrond.jpg",
    chainId: "elrond",
    isEVM: false,
    blockGasLimit: 40000000,
    chainName: "Elrond Mainnet",
    nativeCurrency: {
      name: "Elrond Mainnet",
      symbol: "EGLD",
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://gateway.elrond.com"],
    blockExplorerType: "elrondscan",
    blockExplorerUrls: ["https://explorer.elrond.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.elrond.com",
        apikey: " ",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/transactions/${txid}`,
    accountFormat: (url, address) => `${url}/accounts/${address}`,
    supportedWallets: ["MaiarDefi"],
    txnType: ["Legacy"],
  },

  1313161554: {
    i18nId: "aurora",
    icon: "/asset/image/chains/aurora.svg",
    isEVM: true,
    blockGasLimit: 30000000,
    chainId: "1313161554",
    chainName: "Aurora Mainnet",
    nativeCurrency: {
      name: "Aurora Native Token",
      symbol: "aETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://mainnet.aurora.dev"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://aurorascan.dev", "https://explorer.mainnet.aurora.dev"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.aurorascan.dev",
        apikey: "TQYNSKICRRUXKBZVKN9I3SJJXBJJC8X2EW",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  250: {
    i18nId: "fantom",
    icon: "/asset/image/chains/fantom.svg",
    isEVM: true,
    blockGasLimit: 30000000,
    chainId: "250",
    chainName: "Fantom Opera",
    nativeCurrency: {
      name: "Fantom",
      symbol: "FTM", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://rpcapi.fantom.network/", "https://rpc.fantom.network", "https://rpc2.fantom.network", "https://fantomscan.io/rpc"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://ftmscan.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.ftmscan.com",
        apikey: "3SS28TEVY3XN7YZE7SD499AW9ZWNXBDDE5",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  1284: {
    i18nId: "moonbeam",
    icon: "/asset/image/chains/moonbeam.webp",
    isEVM: true,
    blockGasLimit: 15000000,
    chainId: "1284",
    chainName: "Moonbeam Mainnet",
    nativeCurrency: {
      name: "Moonbeam",
      symbol: "GLMR", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://rpc.api.moonbeam.network"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://moonbeam.moonscan.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api-moonbeam.moonscan.io",
        apikey: "DU8D6GER4BUDEYZ64ERWNWMIG1UD7G5NU9",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  137: {
    chainId: "137",
    i18nId: "polygon",
    icon: "/asset/image/chains/polygon.svg",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Polygon Mainnet",
    nativeCurrency: {
      name: "Matic",
      symbol: "MATIC", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: [
      "https://polygon-rpc.com",
      "https://rpc-mainnet.matic.network",

      // "wss://ws-mainnet.matic.network",
    ],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://polygonscan.com"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.polygonscan.com",
        apikey: "WPWXXPH4H2H1YNYTBJ35CNWPGZF727NS9J",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy", "EIP-1559"],
  },

  42220: {
    chainId: "42220",
    i18nId: "celo",
    isEVM: true,
    blockGasLimit: 20000000,
    chainName: "Celo Mainnet",
    nativeCurrency: {
      name: "Celo",
      symbol: "CELO", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://forno.celo.org"],
    blockExplorerType: "blockscout",
    blockExplorerUrls: ["https://explorer.celo.org"],
    blockExplorerAPIs: [
      {
        endpoint: "https://explorer.celo.org",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/celo.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  1285: {
    chainId: "1285",
    i18nId: "moonriver",
    isEVM: true,
    blockGasLimit: 15000000,
    chainName: "Moonriver",
    nativeCurrency: {
      name: "Moonriver",
      symbol: "MOVR", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://rpc.moonriver.moonbeam.network", "https://moonriver.api.onfinality.io/public"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://moonriver.moonscan.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api-moonriver.moonscan.io",
        apikey: "JS849V13JK4XWPDQ7DIJ71PNSKH3QZBSP4",
      },
    ],

    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/moonriver.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  100: {
    chainId: "100",
    i18nId: "xdai",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Gnosis Chain (formerly xDai)",
    nativeCurrency: {
      name: "Gnosis Chain",
      symbol: "xDAI", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: [
      // "https://rpc.gnosischain.com",
      "https://xdai-archive.blockscout.com",
      "https://xdai-rpc.gateway.pokt.network",
      "https://dai.poa.network",
      // "https://rpc.xdaichain.com"
    ],
    blockExplorerType: "blockscout",
    blockExplorerUrls: ["https://blockscout.com/xdai/mainnet"],
    blockExplorerAPIs: [
      {
        endpoint: "https://blockscout.com/xdai/mainnet",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,

    icon: "/asset/image/chains/gnosis.jpg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  // '25': {
  //   chainId: '25',
  //   i18nId: 'cro',
  //   chainName: 'Cronos Mainnet Beta',
  //   nativeCurrency: {
  //     name: 'Cronos',
  //     symbol: 'CRO', // 2-6 characters long
  //     decimals: 18,
  //   },
  //   rpcUrls: ['https://evm-cronos.crypto.org'],
  //   blockExplorerType: 'blockscout',
  //   blockExplorerUrls: ['https://cronos.crypto.org/explorer'],
  //   blockExplorerAPIs: [
  //     {
  //       endpoint: 'https://cronos.org/explorer',
  //       apikey: '',
  //     },
  //   ],
  //   txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
  //   accountFormat: (url, address) => `${url}/address/${address}`,
  //   icon: "/asset/image/chains/cro.png",
  //   supportedWallets: ['MetaMask', 'WalletConnect'],
  // },

  42161: {
    chainId: "42161",
    i18nId: "etharb",
    isEVM: true,
    blockGasLimit: 200000000,
    chainName: "Arbitrum One",
    nativeCurrency: {
      name: "Arbitrum",
      symbol: "AETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://arb1.arbitrum.io/rpc"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://arbiscan.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api.arbiscan.io",
        apikey: "QEGP657QNPVC18VYTPR5MIPJWKUIN3X27B",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/arbitrum.webp",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  10: {
    chainId: "10",
    i18nId: "op",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Optimistic Ethereum",
    nativeCurrency: {
      name: "Optimistic",
      symbol: "OETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://mainnet.optimism.io/"],
    blockExplorerType: "etherscan",
    blockExplorerUrls: ["https://optimistic.etherscan.io"],
    blockExplorerAPIs: [
      {
        endpoint: "https://api-optimistic.etherscan.io",
        apikey: "VTIXAAAUMMMD31ECNE7MG26626A1A428HW",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/OP.jpg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  592: {
    chainId: "592",
    i18nId: "astar",
    isEVM: true,
    blockGasLimit: 15000000,
    chainName: "Astar Network",
    nativeCurrency: {
      name: "Astar",
      symbol: "ASTR", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: [
      "https://rpc.astar.network:8545",
      // "https://astar.api.onfinality.io/rpc?apikey=2b88efe7-93fb-4a84-aed5-f5c7cd59b3ee", // 502 cors 各种问题
      // "https://astar.api.onfinality.io/public", // 502 cors 各种问题
      // "wss://rpc.astar.network",
    ],
    blockExplorerType: "blockscout",
    blockExplorerUrls: ["https://blockscout.com/astar"],
    blockExplorerAPIs: [
      {
        endpoint: "https://blockscout.com/astar",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/astar.png",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  336: {
    chainId: "336",
    i18nId: "shiden",
    isEVM: true,
    blockGasLimit: 15000000,
    chainName: "Shiden",
    nativeCurrency: {
      name: "Shiden",
      symbol: "SDN", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: [
      "https://rpc.shiden.astar.network:8545",
      // "https://shiden.api.onfinality.io/public",
      // "https://shiden.api.onfinality.io/rpc?apikey=2b88efe7-93fb-4a84-aed5-f5c7cd59b3ee",

      // "wss://shiden.api.onfinality.io/public-ws",
    ],
    blockExplorerType: "blockscout",
    blockExplorerUrls: ["https://blockscout.com/shiden"],
    blockExplorerAPIs: [
      {
        endpoint: "https://blockscout.com/shiden",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/shiden.png",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  66: {
    chainId: "66",
    i18nId: "okex",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "OKExChain Mainnet",
    nativeCurrency: {
      name: "OKExChain",
      symbol: "OKT", // 2-6 characters long
      decimals: 18,
    },
    networkType: 1,
    rpcUrls: ["https://exchainrpc.okex.org"],
    blockExplorerUrls: ["https://www.oklink.com/okexchain"],
    blockExplorerType: "oklink",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/okex.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  256: {
    chainId: "256",
    i18nId: "hecoTest",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Huobi ECO Testnet",
    nativeCurrency: {
      name: "Heco",
      symbol: "HT", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://http-testnet.hecochain.com"],
    blockExplorerUrls: ["https://testnet.hecoinfo.com"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/heco.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy", "EIP-1559"],
  },

  3: {
    chainId: "3",
    i18nId: "ropsten",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "ROPSTEN",
    nativeCurrency: {
      name: "ROPSTEN",
      symbol: "ETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://ropsten.infura.io/v3/c8e6767394f34a06a8f15a0038fbf77a"],
    blockExplorerUrls: ["https://ropsten.etherscan.io"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/eth.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  42: {
    chainId: "42",
    i18nId: "kovan",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Kovan Ether",
    nativeCurrency: {
      name: "Kovan",
      symbol: "ETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://kovan.infura.io/v3/c8e6767394f34a06a8f15a0038fbf77a"],
    blockExplorerUrls: ["https://kovan.etherscan.io"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/eth.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  4: {
    chainId: "4",
    i18nId: "Rinkeby",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Rinkeby Ether",
    nativeCurrency: {
      name: "Rinkeby",
      symbol: "ETH", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://rinkeby.infura.io/v3/c8e6767394f34a06a8f15a0038fbf77a"],
    blockExplorerUrls: ["https://rinkeby.etherscan.io/"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/eth.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },
  97: {
    chainId: "97",
    i18nId: "bnbTest",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "BNB Chain Testnet",
    nativeCurrency: {
      name: "BNB Chain Testnet",
      symbol: "BNB", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://data-seed-prebsc-1-s1.binance.org:8545"],
    blockExplorerUrls: ["https://testnet.bscscan.com"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/binance.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  43113: {
    chainId: "43113",
    i18nId: "avaxTest",
    isEVM: true,
    blockGasLimit: 8000000,
    chainName: "Avax Testnet",
    nativeCurrency: {
      name: "Avax Testnet",
      symbol: "AVAX", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://api.avax-test.network/ext/bc/C/rpc"],
    blockExplorerUrls: ["https://testnet.snowtrace.io"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/avax.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  80001: {
    chainId: "80001",
    i18nId: "polygonTest",
    isEVM: true,
    blockGasLimit: 30000000,
    chainName: "Polygon Testnet",
    nativeCurrency: {
      name: "Polygon Testnet",
      symbol: "MATIC", // 2-6 characters long
      decimals: 18,
    },
    networkType: 0,
    rpcUrls: ["https://matic-mumbai.chainstacklabs.com"],
    blockExplorerUrls: ["https://mumbai.polygonscan.com"],
    blockExplorerType: "etherscan",
    blockExplorerAPIs: [
      {
        endpoint: "",
        apikey: "",
      },
    ],
    txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
    accountFormat: (url, address) => `${url}/address/${address}`,
    icon: "/asset/image/chains/polygon.svg",
    supportedWallets: ["MetaMask", "WalletConnect"],
    txnType: ["Legacy"],
  },

  // "31337": {
  //   chainId: "31337",
  //   i18nId: "local",
  //   isEVM: true,
  //   chainName: "Localhost 8545",
  //   nativeCurrency: {
  //     name: "Local",
  //     symbol: "ETH", // 2-6 characters long
  //     decimals: 18,
  //   },
  //   networkType: 0,
  //   rpcUrls: ["http://127.0.0.1:8545"],
  //   blockExplorerType: "etherscan",
  //   blockExplorerUrls: ["http://localhost"],
  //   blockExplorerAPIs: [
  //     {
  //       endpoint: "http://localhost",
  //       apikey: "",
  //     },
  //   ],
  //   txLinkFormat: (url, txid) => `${url}/tx/${txid}`,
  //   accountFormat: (url, address) => `${url}/address/${address}`,
  //   icon: "/asset/image/chains/localhost.svg",
  //   supportedWallets: ["MetaMask", "WalletConnect"],
  //   txnType: ["Legacy"],
  // },
};
