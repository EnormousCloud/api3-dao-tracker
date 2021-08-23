# api3-dao-tracker

API3 DAO Tracker provides a web interface to see on-chain details of the API3 DAO, including:

- Members, their stakes, shares, voting power and votings history
- Details of the votings
- All events from the smart contracts of the API3 DAO

### Demo links

- https://enormous.cloud/dao/api3/tracker/
- [Public DAO Test on Rinkeby](https://enormous.cloud/dao/api3/tracker-rinkeby/)

### Changes coming in the nearest time
- [x] User-friendly event history (compacted, colourful and without pennies)
- [x] Display DAO treasuries balances
- [ ] Hourly re-checks of ENS, vote scripts and treasuries instead of checks on start
- [ ] Include shares/rewards/stakes in rewards snapshots and display both stakes in wallet history
- [ ] Match total number of shares (check unstaking cases)
- [ ] Finish mobile look
- [x] Fix message about staking target for the lowest and highest value
- [x] Clean up code for vote script parsing
- [ ] Votes: group into PENDING/EXECUTED/REJECTED, order would better be by reverse start date. Current order is incorrect
- [ ] Votes: missing the date of expiration and date of execution
- [ ] Improve filter for DAO members by classificaiton
- [ ] CSV export: Votings, Votes, Wallets, Rewards, Delegations, Events
- [ ] Prometheus metrics

### Running locally

- This tool is built on Rust, and to build it locally you need [Rust toolchain](https://www.rust-lang.org/tools/install)
- To run this tool locally, you need the connection to gETH IPC endpoint. The tool doesn't do polling from HTTP endpoint (at least yet)
- The tool contains `client` and `server`. 
- To build `client`, you need [trunkrs.dev](https://github.com/thedodd/trunk) (which is an alternative to webpack) distribution, and it should be simply `trunk build` to prepare assets for distribution.
- After your `client/dist` folder is ready, copy environment variables nito `.env` from the environment you want to work with, mainnet or rinkeby
- After that `server` could be run with `cargo run --release`.
- The most important - you also need to have patience to wait for all previous events to be cached ;). Please make sure `CACHE_DIR` folder was set up and mentioned as environment variable properly. Downloaded batches of events will be saved, so time on the next run would be less (though it would be still a few minutes for every day of the history).
- It would be useful to review `run.sh` file, it contains exact scripts that are used for building and deployments

### Disclaimer

- This is a work in progress. 
- The numbers might not match to the penny as the implementation of calculations are not identical to the original smart contract.
- Please [create an issue](https://github.com/EnormousCloud/api3-dao-tracker/issues) in case of proposals or questions.
`
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
`

### License
MIT
