/**
 * Example JavaScript code that interacts with the page and Web3 wallets
 */

// esm import
// import { ethers } from "ethers";
// import Fortmatic from "fortmatic";
// import Web3Modal from "web3modal";
// import WalletConnectProvider from "ethereum-provider";

const ethers = window.ethers;
const Fortmatic = window.Fortmatic;
const Web3Modal = window.Web3Modal;
const WalletConnectProvider = window.WalletConnectProvider.default;

// Web3modal instance
let modal;

// Chosen wallet provider given by the dialog window
let provider;

// Web3Provider
let web3;

// Address of the selected account
let selectedAccount;

/**
 * Setup the orchestra
 */
function init() {
  console.log("Initializing example");
  // console.log("WalletConnectProvider is", WalletConnectProvider);
  // console.log("Fortmatic is", Fortmatic);
  // console.log("window.ethereum is", window.ethereum);

  // Check that the web page is run in a secure context, as otherwise MetaMask won't be available
  //   if (location.protocol !== "https:") {
  //     // https://ethereum.stackexchange.com/a/62217/620
  //     const alert = document.querySelector("#alert-error-https");
  //     alert.style.display = "block";
  //     document.querySelector("#btn-connect").setAttribute("disabled", "disabled");
  //     return;
  //   }

  // Tell Web3modal what providers we have available.
  // Built-in web browser provider (only one can exist as a time)
  // like MetaMask, Brave or Opera is added automatically by Web3modal
  const providerOptions = {
    walletconnect: {
      package: WalletConnectProvider,
      options: {
        infuraId: "9aa3d95b3bc440fa88ea12eaa4456161",
      },
    },

    fortmatic: {
      package: Fortmatic,
      options: {
        key: "",
      },
    },
  };

  modal = new Web3Modal.default({
    cacheProvider: false, // optional
    providerOptions, // required
    disableInjectedProvider: false, // optional. For MetaMask / Brave / Opera.
  });

  console.log("Web3Modal instance is", modal);
}

/**
 * Kick in the UI action after Web3modal dialog has chosen a provider
 */
async function fetchAccountData() {
  if (!provider) return;

  // Get a Web3 instance for the wallet
  web3 = new ethers.providers.Web3Provider(provider);
  console.log("Web3 instance is", web3);

  // Get connected chain id from Ethereum node
  const network = await web3.getNetwork();
  // Load chain information over an HTTP API
  console.log("network is", network);
  //   document.querySelector("#network-name").textContent = chainData.name;

  // MetaMask does not give you all accounts, only the selected account
  const accounts = await web3.listAccounts();
  selectedAccount = accounts[0];
  console.log("Got accounts", accounts, selectedAccount);

  // const signer = web3.getSigner();
  // selectedAccount = await signer.getAddress();

  //   document.querySelector("#selected-account").textContent = selectedAccount;

  // Get a handle
  //   const template = document.querySelector("#template-balance");
  //   const accountContainer = document.querySelector("#accounts");

  //   // Purge UI elements any previously loaded accounts
  //   accountContainer.innerHTML = "";

  // Go through all accounts and get their ETH balance
  const rowResolvers = accounts.map(async address => {
    // const balanceRaw = ethers.utils.formatUnits(balance);
    // const balanceFormat = (+balanceRaw).toFixed(2);
    const balance = await web3.getBalance(address);
    // ethBalance is a BigNumber instance
    // https://github.com/indutny/bn.js/
    const ethBalance = ethers.utils.formatUnits(balance, "ether");
    const humanFriendlyBalance = parseFloat(ethBalance).toFixed(4);
    console.log("humanFriendlyBalance", humanFriendlyBalance);
    // Fill in the templated row and put in the document
    // const clone = template.content.cloneNode(true);
    // clone.querySelector(".address").textContent = address;
    // clone.querySelector(".balance").textContent = humanFriendlyBalance;
    // accountContainer.appendChild(clone);
  });

  // Because rendering account does its own RPC communication
  // with Ethereum node, we do not want to display any results
  // until data for all accounts is loaded
  await Promise.all(rowResolvers);

  // Display fully loaded UI for wallet data
  //   document.querySelector("#prepare").style.display = "none";
  //   document.querySelector("#connected").style.display = "block";
}

/**
 * Fetch account data for UI when
 * - User switches accounts in wallet
 * - User switches networks in wallet
 * - User connects wallet initially
 */
async function refreshAccountData() {
  // If any current data is displayed when
  // the user is switching accounts in the wallet
  // immediate hide this data
  //   document.querySelector("#connected").style.display = "none";
  //   document.querySelector("#prepare").style.display = "block";

  // Disable button while UI is loading.
  // fetchAccountData() will take a while as it communicates
  // with Ethereum node via JSON-RPC and loads chain data
  // over an API call.
  //   document.querySelector("#btn-connect").setAttribute("disabled", "disabled");
  await fetchAccountData(provider);
  //   document.querySelector("#btn-connect").removeAttribute("disabled");
}

/**
 * Connect wallet button pressed.
 */
export async function connectHandle() {
  console.log("Opening a dialog", modal);
  try {
    provider = await modal.connect();
    window.ethereum = provider;
    console.log("connectHandle provider =>", provider);
  } catch (e) {
    console.log("Could not get a wallet connection", e);
    return;
  }

  // Automatically Refresh on Network Change
  provider.on("network", (newNetwork, oldNetwork) => {
    // When a Provider makes its initial connection, it emits a "network"
    // event with a null oldNetwork along with the newNetwork. So, if the
    // oldNetwork exists, it represents a changing network
    if (oldNetwork) {
      window.location.reload();
    }
  });

  provider.on("disconnect", async () => {
    console.log("disconnect");
    disconnectHandle(true);
  });

  // Subscribe to accounts change
  provider.on("accountsChanged", accounts => {
    console.log("accountsChanged", accounts);
    fetchAccountData();
  });

  // Subscribe to chainId change
  provider.on("chainChanged", chainId => {
    console.log("chainChanged", chainId);
    fetchAccountData();
  });

  await refreshAccountData();
}

/**
 * Disconnect wallet button pressed.
 */
export async function disconnectHandle(isServerClose) {
  console.log("Killing the wallet connection", provider);

  if (!isServerClose && provider && provider.close) {
    provider.removeAllListeners();
    await provider.close();
  }

  // If the cached provider is not cleared,
  // WalletConnect will default to the existing session
  // and does not allow to re-scan the QR code with a new wallet.
  // Depending on your use case you may want or want not his behavior.
  await modal.clearCachedProvider();
  provider = null;
  window.ethereum = null;
  selectedAccount = null;

  // Set the UI back to the initial state
  //   document.querySelector("#prepare").style.display = "block";
  //   document.querySelector("#connected").style.display = "none";
}
/**
 * Main entry point.
 */
window.addEventListener("load", async () => {
  init();
  //   document.querySelector("#btn-connect").addEventListener("click", connectHandle);
  //   document.querySelector("#btn-disconnect").addEventListener("click", disconnectHandle);
});

// 预估gas, 并且调用合约
// functionName:你要调用的方法,如safeMint
// args: 调用合约的参数, array类型, 例如[arg1, arg2, arg3, {}], 最后的{}不能少
// 函数中会将计算出来的gas要放里面, 如果要传入eth, 则最后的{}为{value:xxx}
async function executeContractMethodWithEstimatedGas(contractAddress, contractAbi, functionName, args) {
  if (!provider) {
    console.log("please connect a wallet first");
    return;
  }
  const contract = new ethers.Contract(contractAddress, contractAbi, provider);

  const estimatedGas = await contract.estimateGas[functionName](...args)
    .then(value => {
      const minGas = ethers.BigNumber.from("300000");
      if (value.lt(minGas)) {
        return minGas;
      }
      return value;
    })
    .catch(error => {
      // 出错时给个固定值
      return ethers.BigNumber.from("700000");
    });
  console.log("estimatedGas", estimatedGas);
  // 将计算结果放入参数中
  const argsForOverridden = args.pop();
  args.gasLimit = estimatedGas.times(1.2);
  args.push(argsForOverridden);
  return contract.connect(getSigner())[functionName](...args);
}

// tx是上面executeContractMethodWithEstimatedGas会返回的结果
async function waitForTransaction(tx) {
  //2是网络确认数量，自定义
  return await provider.waitForTransaction(tx.hash, 2);
}

// 以下是周边函数
function getSigner(web3) {
  if (!web3) {
    console.log("please connect a wallet first");
    return;
  }
  return web3.getSigner();
}
