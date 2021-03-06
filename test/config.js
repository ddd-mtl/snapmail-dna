//const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaPath = path.join(__dirname, "../dist/snapmail-dna.dna.json")
const dna = Config.dna(dnaPath, 'snapmail-test')
//const dna2 = Config.dna(dnaPath, 'app-spec', { uuid: 'altered-dna' })

const networkType = 'sim2h'
const network =
    ( networkType === 'websocket'
            ? Config.network('websocket')

            : networkType === 'memory'
                ? Config.network('memory')

                : networkType === 'sim2h'
                    ? {
                        type: 'sim2h',
                        sim2h_url: 'ws://sim2h.harris-braun.com:9051'
                        //sim2h_url: 'ws://localhost:9000'
                    }

                    : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
    )

const networkOffline = {
    type: 'sim2h',
    sim2h_url: 'ws://bogus:4242'
}

const perfLogger = {
    type: 'warn',
    state_dump: false,
}

const logger = {
    type: 'debug',
    rules: {
        rules: [
            {
                exclude: true,
                pattern: '.*parity.*'
            },
            {
                exclude: true,
                pattern: '.*mio.*'
            },
            {
                exclude: true,
                pattern: '.*tokio.*'
            },
            {
                exclude: true,
                pattern: '.*hyper.*'
            },
            {
                exclude: true,
                pattern: '.*rusoto_core.*'
            },
            {
                exclude: true,
                pattern: '.*want.*'
            },
            {
                exclude: true,
                pattern: '.*rpc.*'
            }
        ]
    },
    state_dump: true
}

const tracing = ({playerName}) => ({
    type: 'jaeger',
    service_name: `holochain-${playerName}`
})

const commonConfig = { logger, network/*, tracing*/ }

const perfConfig = { perfLogger, network }


module.exports = {
    conductorConfig: Config.gen({
            app: dna
        },
        commonConfig
    ),
    offline: Config.gen({
            app: dna
        },
        { logger, network: networkOffline, tracing }
    ),
    conductorConfigPerf: Config.gen({
            app: dna
        },
        perfConfig
    ),
    // , two: Config.gen({
    //         app1: dna,
    //         app2: dna2
    //     },
    //     {
    //         bridges: [
    //             Config.bridge('test-bridge', 'app1', 'app2')
    //         ],
    //         ...commonConfig
    //     }),
}
