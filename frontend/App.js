import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { getGreetingFromContract, setGreetingOnContract, verifiedGreetingOnContract } from './near-api';
import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';
import { generateCalldata } from './generate-proof';


export default function App() {
  const [valueFromBlockchain, setValueFromBlockchain] = React.useState();

  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  // Get blockchian state once on component load
  React.useEffect(() => {
    getGreetingFromContract()
      .then(setValueFromBlockchain)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }, []);

  /// If user not signed-in with wallet - show prompt
  if (!window.walletConnection.isSignedIn()) {
    // Sign-in flow will reload the page later
    return <SignInPrompt greeting={valueFromBlockchain} />;
  }

  function changeGreeting(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { greetingInput } = e.target.elements;
    setGreetingOnContract(greetingInput.value)
      .then(getGreetingFromContract)
      .then(setValueFromBlockchain)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }

  async function verifyGreeting(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { greetingInput, a, b } = e.target.elements;

    const { proof, inputs } = await generateCalldata({ a: a.value, b: b.value })
    console.log("Proof:", proof);
    console.log("Inputs:", inputs);

    verifiedGreetingOnContract(greetingInput.value, proof, inputs)
      .then(getGreetingFromContract)
      .then(setValueFromBlockchain)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });

    setUiPleaseWait(false);
  }

  return (
    <>
      <SignOutButton accountId={window.accountId} />
      <main className={uiPleaseWait ? 'please-wait' : ''}>
        <h1>
          The contract says: <span className="greeting">{valueFromBlockchain}</span>
        </h1>
        <form onSubmit={verifyGreeting} className="change">
          <div>
            <div>
              <label>Change greeting:</label>
              <input
                autoComplete="off"
                defaultValue={valueFromBlockchain}
                id="greetingInput"
              />
            </div>
            <div>
              <label>Circuit inputs:</label><div>
                <input
                  autoComplete="off"
                  defaultValue={3}
                  id="a"
                />
                <input
                  autoComplete="off"
                  defaultValue={11}
                  id="b"
                />
              </div>
            </div>
          </div>
          <div>
            <button>
              <span>Save</span>
              <div className="loader"></div>
            </button>
          </div>
        </form>
        <EducationalText />
      </main>
    </>
  );
}
