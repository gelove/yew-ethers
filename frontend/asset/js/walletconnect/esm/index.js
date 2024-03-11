import { ethers } from "ethers";
import WalletConnectProvider from "ethereum-provider";

const { ethereum } = window;

console.log("ethers", ethers);

// 需要钱包连接的方法交由js处理
let account;
let _provider = null;
let _w3 = null;

export async function initProvider() {
  if (!_provider) {
    _provider = new WalletConnectProvider({
      rpc: {
        1: "https://cloudflare-eth.com/", //https://ethereumnodes.com/
        137: "https://polygon-rpc.com/", //https://docs.polygon.technology/docs/develop/network-details/network/
      },
      // bridge: 'https://bridge.walletconnect.org',
    });
    await _provider.enable();
  }
  return _provider;
}

export async function initWeb3() {
  if (!_w3) {
    const provider = await initProvider();
    _w3 = new ethers.providers.Web3Provider(provider);
  }
  return _w3;
}

export async function connectProvider() {
  try {
    _provider = new WalletConnectProvider({
      rpc: {
        1: "https://cloudflare-eth.com/", //https://ethereumnodes.com/
        137: "https://polygon-rpc.com/", //https://docs.polygon.technology/docs/develop/network-details/network/
      },
      // bridge: 'https://bridge.walletconnect.org',
    });
    await _provider.enable();
    const web3 = new ethers.providers.Web3Provider(_provider);

    // _provider = await initProvider();
    // const web3 = await initWeb3();
    console.log("Web3 instance is", web3);
    // Get connected chain id from Ethereum node
    const network = await web3.getNetwork();
    // Load chain information over an HTTP API
    // const chainData = evmChains.getChain(network.chainId);
    // document.querySelector("#network-name").textContent = chainData.name;

    // MetaMask does not give you all accounts, only the selected account
    const accounts = await web3.listAccounts();
    if (Array.isArray(accounts) && accounts.length > 0) {
      account = accounts[0];
    }
    console.log("Got accounts", accounts, account);
  } catch (e) {
    console.error("connectProvider error ", e);
    await disconnectProvider();
  }
}

export async function disconnectProvider() {
  if (_provider?.close) {
    console.log("disconnected provider close");
    await _provider.close();
  }
  if (_provider?.disconnect) {
    console.log("disconnected provider disconnect");
    await _provider.disconnect();
  }
  _provider = null;
  console.log("disconnected");
}

// var sign = async msg => {
//   if (w3) {
//     return await w3.eth.personal.sign(msg, account);
//   } else {
//     return false;
//   }
// };

// var contract = async (abi, address) => {
//   if (w3) {
//     return new w3.eth.Contract(abi, address);
//   } else {
//     return false;
//   }
// };

// var address = "0xDaB040aaCBc7A06C3196Dfd1e510f23cB66E4726";
// var abi = [
//   { inputs: [], name: "count", outputs: [{ internalType: "uint256", name: "", type: "uint256" }], stateMutability: "view", type: "function" },
//   { inputs: [], name: "increment", outputs: [{ internalType: "uint256", name: "", type: "uint256" }], stateMutability: "nonpayable", type: "function" },
// ];

const ErrorHandler = error => {
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

export class MetamaskConnector {
  constructor() {
    this._provider = null;
    this.#_init();
  }

  #_init() {
    if (ethereum) {
      this._provider = new ethers.providers.Web3Provider(ethereum);
    } else {
      console.log("Metamask install karle BSDK");
      this._provider = null;
      // open metamask install page
      //   throw new Error("Metamask install karle BSDK");
    }
  }

  async _connect() {
    try {
      if (!this._provider) this.#_init();
      const accounts = await this._provider.send("eth_requestAccounts", []);
      const network = await this._provider.getNetwork();
      return {
        message: "Wallet connected",
        success: true,
        address: accounts[0],
        chainId: network.chainId,
      };
    } catch (err) {
      return {
        message: ErrorHandler(err),
        success: false,
        address: null,
        chainId: null,
      };
    }
  }

  onChainChanged(callback) {
    if (!ethereum) return;
    ethereum.on("chainChanged", chainId => {
      callback(chainId);
    });
  }

  onAccountChange(callback) {
    if (!ethereum) return;
    ethereum.on("accountsChanged", accounts => {
      callback(accounts);
    });
  }

  async _signMessage(message) {
    try {
      const signer = this._provider.getSigner();
      await signer.signMessage(message);
      return {
        message: "Message signed",
        success: true,
      };
    } catch (err) {
      return {
        message: ErrorHandler(err),
        success: false,
      };
    }
  }

  removeListeners() {
    if (ethereum && "removeAllListeners" in ethereum) {
      ethereum.removeAllListeners();
    }
  }

  getProvider() {
    return this._provider;
  }

  async addChain(chainParams) {
    // {
    //   method: "wallet_addEthereumChain",
    //   params: [
    //     {
    //       chainName: "BSC Testnet",
    //       chainId: "0x61",
    //       nativeCurrency: { name: "tBNB", decimals: 18, symbol: "tBNB" },
    //       rpcUrls: ["https://data-seed-prebsc-1-s1.binance.org:8545"],
    //     },
    //   ],
    // }
    await ethereum.request(chainParams);
  }

  async _switchNetwork(chainId) {
    try {
      await this._provider.send("wallet_switchEthereumChain", [
        {
          chainId: `0x${chainId.toString(16)}`,
        },
      ]);
      return {
        message: "Network switched",
        success: true,
      };
    } catch (error) {
      if (error.code === 4902) {
        const list = [];
        if (!list[chainId]) {
          return {
            message: ErrorHandler(error),
            success: false,
          };
        }
        await this.addChain(list[chainId]);
        return;
      }
      if (error.code === -32002) {
        return {
          success: false,
          message: ErrorHandler(error),
        };
      }
      if (error.code === "SERVER_ERROR") {
        return {
          success: false,
          message: ErrorHandler(error),
        };
      }
      return {
        success: false,
        message: ErrorHandler(error),
      };
    }
  }

  _contract(address, abi) {
    if (!this._provider) return;
    return new ethers.Contract(address, abi, this._provider);
  }
}

export class WalletConnector {
  constructor({ rpc }) {
    this.rpc = rpc;
    this._provider = null;
    this._web3Provider = null;
    this.#_init();
  }

  #_init() {
    this._provider = new WalletConnectProvider({
      rpc: this.rpc, // Required
      qrcodeModalOptions: {
        mobileLinks: ["rainbow", "metamask", "trust"],
      },
    });
    this._web3Provider = new ethers.providers.Web3Provider(this._provider);
  }

  async _connect() {
    try {
      // Enable session (triggers QR Code modal)
      if (!this._provider) this.#_init();
      await this._provider.enable();
      const signer = this._web3Provider.getSigner();
      const address = await signer.getAddress();
      const chainId = await this._web3Provider.getNetwork();
      return {
        message: "Wallet connected",
        success: true,
        address: address,
        chainId: chainId.chainId,
      };
    } catch (err) {
      return {
        message: ErrorHandler(err),
        success: false,
        address: null,
        chainId: null,
      };
    }
  }

  async _disconnectWC() {
    await this._provider.disconnect();
    this._provider = null;
  }

  onChainChanged(callback) {
    this._provider.on("chainChanged", chainId => {
      callback(chainId);
    });
  }

  onAccountChange(callback) {
    this._provider.on("accountsChanged", accounts => {
      callback(accounts);
    });
  }

  async _signMessage(message) {
    try {
      const signer = this._web3Provider.getSigner();
      await signer.signMessage(message);
      return {
        success: true,
        message: "Message signed",
      };
    } catch (err) {
      return {
        success: false,
        message: ErrorHandler(err),
      };
    }
  }

  getProvider() {
    return this._web3Provider;
  }

  async _switchNetwork(chainId) {
    try {
      let id = `0x${chainId.toString(16)}`;
      await this._web3Provider.send("wallet_switchEthereumChain", [
        {
          chainId: id,
        },
      ]);
      this.#_init();
      return {
        message: "Network switched",
        success: true,
      };
    } catch (error) {
      if (error.code === 4902) {
        return {
          message: ErrorHandler(error),
          success: false,
        };
      }
      if (error.code === -32002) {
        return {
          success: false,
          message: ErrorHandler(error),
        };
      }
      if (error.code === "SERVER_ERROR") {
        return {
          success: false,
          message: ErrorHandler(error),
        };
      }
      return {
        success: false,
        message: ErrorHandler(error),
      };
    }
  }

  _contract(address, abi) {
    if (!this._web3Provider) return;
    return new ethers.Contract(address, abi, this._web3Provider);
  }
}
