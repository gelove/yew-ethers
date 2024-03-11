let provider = null;
let web3 = null;
let account = null;

export function initProvider() {
  if (!provider) {
    provider = new WalletConnectProvider.default({
      rpc: {
        1: "https://cloudflare-eth.com/",
        5: "https://goerli.prylabs.net/",
        137: "https://polygon-rpc.com/",
      },
      // bridge: 'https://bridge.walletconnect.org',
    });
    console.log("initProvider");
  }
  return provider;
}

export const initWeb3 = () => {
  web3 = new ethers.providers.Web3Provider(provider);
};

export const connect = async () => {
  try {
    initProvider();
    await provider.enable();
    initWeb3();
    const network = await web3.getNetwork();
    console.log("network =>", network, network.chainId);
    const accounts = await web3.listAccounts();
    console.log("accounts =>", accounts);
    account = accounts[0];
    const signer = web3.getSigner();
    account = await signer.getAddress();
    console.log("account", account);
    onChainChange(chainId => {
      console.log("chainId =>", chainId);
      initWeb3();
    });
  } catch (e) {
    console.error("connect error ", e);
    await disconnect();
  }
};

// Close provider session
export const disconnect = async () => {
  console.log("disconnect =>", provider.close, provider.disconnect);
  if (provider) {
    await provider.disconnect();
  }
  provider = null;
  web3 = null;
  console.log("disconnected");
};

export const sign = async msg => {
  if (!web3) {
    return false;
  }
  const signer = web3.getSigner();
  return await signer.signMessage(message);
};

export const contract = async (address, abi) => {
  if (!web3) {
    return false;
  }
  return new ethers.Contract(address, abi, web3);
};

export const onChainChange = callback => {
  provider.on("chainChanged", chainId => {
    callback(chainId);
  });
};

export const onAccountChange = callback => {
  provider.on("accountsChanged", accounts => {
    callback(accounts);
  });
};

export const ErrorHandler = error => {
  let msg = "";
  if (error.code === 4001) {
    msg = "Request rejected. Please accept to continue.";
  } else if (error.code === 4100) {
    msg = "Account is inaccessible. Please unlock your account.";
  } else if (error.code === 4902) {
    msg = "Network is not supported. Please add the network manually.";
  } else if (error.code === -32002) {
    msg = "Network switch request is already pending, please approve the request on your wallet.";
  } else if (error.code === "SERVER_ERROR") {
    msg = "Please switch network in your wallet manually.";
  } else {
    msg = "error: " + error.message || "Something is wrong. Try reloading!";
  }
  return msg;
};

// class MetamaskConnector {
//   constructor() {
//     this._provider = null;
//     this.#_init();
//   }

//   #_init() {
//     if (window.ethereum) {
//       this._provider = new ethers.providers.Web3Provider(window.ethereum);
//     } else {
//       console.log("Metamask install karle BSDK");
//       this._provider = null;
//       // open metamask install page
//       //   throw new Error("Metamask install karle BSDK");
//     }
//   }

//   async _connect() {
//     try {
//       if (!this._provider) this.#_init();
//       const accounts = await this._provider.send("eth_requestAccounts", []);
//       const network = await this._provider.getNetwork();
//       return {
//         message: "Wallet connected",
//         success: true,
//         address: accounts[0],
//         chainId: network.chainId,
//       };
//     } catch (err) {
//       return {
//         message: ErrorHandler(err),
//         success: false,
//         address: null,
//         chainId: null,
//       };
//     }
//   }

//   onChainChanged(callback) {
//     if (!window.ethereum) return;
//     window.ethereum.on("chainChanged", chainId => {
//       callback(chainId);
//     });
//   }

//   onAccountChange(callback) {
//     if (!window.ethereum) return;
//     window.ethereum.on("accountsChanged", accounts => {
//       callback(accounts);
//     });
//   }

//   async _signMessage(message) {
//     try {
//       const signer = this._provider.getSigner();
//       await signer.signMessage(message);
//       return {
//         message: "Message signed",
//         success: true,
//       };
//     } catch (err) {
//       return {
//         message: ErrorHandler(err),
//         success: false,
//       };
//     }
//   }

//   removeListeners() {
//     if (window.ethereum && "removeAllListeners" in window.ethereum) {
//       window.ethereum.removeAllListeners();
//     }
//   }

//   getProvider() {
//     return this._provider;
//   }

//   async addChain(chainParams) {
//     // {
//     //   method: "wallet_addEthereumChain",
//     //   params: [
//     //     {
//     //       chainName: "BSC Testnet",
//     //       chainId: "0x61",
//     //       nativeCurrency: { name: "tBNB", decimals: 18, symbol: "tBNB" },
//     //       rpcUrls: ["https://data-seed-prebsc-1-s1.binance.org:8545"],
//     //     },
//     //   ],
//     // }
//     await window.ethereum.request(chainParams);
//   }

//   async _switchNetwork(chainId) {
//     try {
//       await this._provider.send("wallet_switchEthereumChain", [
//         {
//           chainId: `0x${chainId.toString(16)}`,
//         },
//       ]);
//       return {
//         message: "Network switched",
//         success: true,
//       };
//     } catch (error) {
//       if (error.code === 4902) {
//         const list = [];
//         if (!list[chainId]) {
//           return {
//             message: ErrorHandler(error),
//             success: false,
//           };
//         }
//         await this.addChain(list[chainId]);
//         return;
//       }
//       if (error.code === -32002) {
//         return {
//           success: false,
//           message: ErrorHandler(error),
//         };
//       }
//       if (error.code === "SERVER_ERROR") {
//         return {
//           success: false,
//           message: ErrorHandler(error),
//         };
//       }
//       return {
//         success: false,
//         message: ErrorHandler(error),
//       };
//     }
//   }

//   _contract(address, abi) {
//     if (!this._provider) return;
//     return new ethers.Contract(address, abi, this._provider);
//   }
// }

// class WalletConnector {
//   constructor({ rpc }) {
//     this.rpc = rpc;
//     this._provider = null;
//     this._web3Provider = null;
//     this.#_init();
//   }

//   #_init() {
//     this._provider = new WalletConnectProvider({
//       rpc: this.rpc, // Required
//       qrcodeModalOptions: {
//         mobileLinks: ["rainbow", "metamask", "trust"],
//       },
//     });
//     this._web3Provider = new ethers.providers.Web3Provider(this._provider);
//   }

//   async _connect() {
//     try {
//       // Enable session (triggers QR Code modal)
//       if (!this._provider) this.#_init();
//       await this._provider.enable();
//       const signer = this._web3Provider.getSigner();
//       const address = await signer.getAddress();
//       const chainId = await this._web3Provider.getNetwork();
//       return {
//         message: "Wallet connected",
//         success: true,
//         address: address,
//         chainId: chainId.chainId,
//       };
//     } catch (err) {
//       return {
//         message: ErrorHandler(err),
//         success: false,
//         address: null,
//         chainId: null,
//       };
//     }
//   }

//   async _disconnectWC() {
//     await this._provider.disconnect();
//     this._provider = null;
//   }

//   onChainChanged(callback) {
//     this._provider.on("chainChanged", chainId => {
//       callback(chainId);
//     });
//   }

//   onAccountChange(callback) {
//     this._provider.on("accountsChanged", accounts => {
//       callback(accounts);
//     });
//   }

//   async _signMessage(message) {
//     try {
//       const signer = this._web3Provider.getSigner();
//       await signer.signMessage(message);
//       return {
//         success: true,
//         message: "Message signed",
//       };
//     } catch (err) {
//       return {
//         success: false,
//         message: ErrorHandler(err),
//       };
//     }
//   }

//   getProvider() {
//     return this._web3Provider;
//   }

//   async _switchNetwork(chainId) {
//     try {
//       let id = `0x${chainId.toString(16)}`;
//       await this._web3Provider.send("wallet_switchEthereumChain", [
//         {
//           chainId: id,
//         },
//       ]);
//       this.#_init();
//       return {
//         message: "Network switched",
//         success: true,
//       };
//     } catch (error) {
//       if (error.code === 4902) {
//         return {
//           message: ErrorHandler(error),
//           success: false,
//         };
//       }
//       if (error.code === -32002) {
//         return {
//           success: false,
//           message: ErrorHandler(error),
//         };
//       }
//       if (error.code === "SERVER_ERROR") {
//         return {
//           success: false,
//           message: ErrorHandler(error),
//         };
//       }
//       return {
//         success: false,
//         message: ErrorHandler(error),
//       };
//     }
//   }

//   _contract(address, abi) {
//     if (!this._web3Provider) return;
//     return new ethers.Contract(address, abi, this._web3Provider);
//   }
// }
