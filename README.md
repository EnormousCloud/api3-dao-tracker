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

### Disclaimer

- This is a work in progress. 
- The numbers might not match to the penny as the implementation of calculations are not identical to the original smart contract.
- Please [create an issue](https://github.com/EnormousCloud/api3-dao-tracker/issues) in case of proposals or questions.
`
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
`

### License
MIT DAO staking target is reached, so APR will be decreased by 1% for the next epoch until it reaches 2.5% 
