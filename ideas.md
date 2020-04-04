# screepsctl


Need some kind of central intelligence


Emperor -> Duke -> Baron
Shard   -> Room -> Spawn



## Initial plan

* Harvester
	* Energy
		* Feeder (give to spawn/extensions/storage)
			* Need smarts to go to closest source with open spot (or with spot open soonest?)
	* Minerals
		* This is later, need levels first
* Upgrader
	* Take from spawn, use to upgrade controller
* Builder
	* Primary duty: fix things degraded past some amount, tbd
	* Secondary duty: build new items



## Other ideas

* Build roads between well travelled places
	* Once there are X harvesters, build road from storage to energy source & minerals

* metrics
	* rolling average, `x` resources per tick for various resources
	* rolling average 1 `creep type` every `x` seconds

* use a different logger that i can change the level of at runtime 
	* (ie by reading a memory value i can set from the command line)

* can i write to the network? so that i could get logs remotely?

### Responding to threats

Low level attacks can probably be handled by walls/ramparts and turrets
If a significant or extended attack happens, need to adjust production quickly to respond, assign more creeps to fixing or constructing 