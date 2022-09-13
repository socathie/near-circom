import { Worker, NEAR, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

import { readFileSync } from 'fs';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('returns the default greeting', async (t) => {
  const { contract } = t.context.accounts;
  const message: string = await contract.view('get_greeting', {});
  t.is(message, 'Hello');
});

test('changes the message', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'set_greeting', { message: 'Howdy' });
  const message: string = await contract.view('get_greeting', {});
  t.is(message, 'Howdy');
});

test('verify proof', async (t) => {
  const { contract } = t.context.accounts;

  const proof_str: string = readFileSync('../contract/circuits/proof.json', 'utf-8');
  const pub_input_str: string = readFileSync('../contract/circuits/public.json', 'utf-8');
  const res: boolean = await contract.view('verify_proof_on_chain', { proof: proof_str, inputs: pub_input_str });

  t.is(res, true);
});

test('invalid proof', async (t) => {
  const { contract } = t.context.accounts;

  const proof_str: string = readFileSync('../contract/circuits/proof.json', 'utf-8');
  const pub_input_str: string = '["0"]';
  const res: boolean = await contract.view('verify_proof_on_chain', { proof: proof_str, inputs: pub_input_str });

  t.is(res, false);
});

test('verified set greeting ', async (t) => {
  const { root, contract } = t.context.accounts;

  const proof_str: string = readFileSync('../contract/circuits/proof.json', 'utf-8');
  const pub_input_str: string = readFileSync('../contract/circuits/public.json', 'utf-8');

  await root.call(contract, 'set_verified_greeting', { message: 'Howdy', proof: proof_str, inputs: pub_input_str }, { gas: "300000000000000" });
  const message: string = await contract.view('get_greeting', {});

  t.is(message, 'Howdy');
});


test('invalid set greeting ', async (t) => {
  const { root, contract } = t.context.accounts;

  const proof_str: string = readFileSync('../contract/circuits/proof.json', 'utf-8');
  const pub_input_str: string = '["0"]';

  await root.call(contract, 'set_verified_greeting', { message: 'Howdy', proof: proof_str, inputs: pub_input_str }, { gas: "300000000000000" })
    .catch((error) => t.is(String(error).includes("invalid proof"), true));
});