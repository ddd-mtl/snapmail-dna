const { conductorConfig, conductorConfigPerf } = require('../config')
const { sleep, filterMailList } = require('../utils')

// -- Export scenarios -- //

module.exports = scenario => {
    //scenario("test stress 10 mail", test_stress_10_mail)
    scenario("test stress 100 mail", test_stress_100_mail)


    // LONG LONG TESTS
    // hdk::query() takes too long (over 90sec)
    //scenario("test stress 1k mail", test_stress_1k_mail)
}

// -- Scenarios -- //

const test_stress_1k_mail = async (s, t) => {
    await test_stress_send_mail(s, t, 1000)
}

const test_stress_100_mail = async (s, t) => {
    await test_stress_send_mail(s, t, 100)
}

const test_stress_10_mail = async (s, t) => {
    await test_stress_send_mail(s, t, 10)
}

/**
 *
 */
const test_stress_send_mail = async (s, t, loop_count) => {

    const {alex, billy} = await s.players({alex: conductorConfigPerf, billy: conductorConfigPerf}, true)

    let send_start = Date.now();

    let send_params;
    let send_result;
    for (let i = 0; i < loop_count; i++) {
        send_params = {
            subject: "test-outmail " + i,
            payload: "blablabla " + i,
            to: [alex.info('app').agentAddress],
            cc: [],
            bcc: [],
            manifest_address_list: []
        }
        console.log('** Sending ' + i)
        send_result = await billy.call("app", "snapmail", "send_mail", send_params)
        console.log('send_result: ' + JSON.stringify(send_result.Ok))
        t.deepEqual(send_result.Ok.to_pendings, {}) // Should receive via DM, so no pendings
    }
    await s.consistency()
    let send_end = Date.now();
    let send_duration = (send_end - send_start) / 1000
    console.log("Send duration: " + send_duration + ' sec')

    let get_all_start = Date.now();
    const arrived_result = await alex.call("app", "snapmail", "get_all_arrived_mail", {})
    console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok.length))
    const arrived_mail_list = arrived_result.Ok;
    t.deepEqual(arrived_mail_list.length, loop_count)

    let get_all_end = Date.now();
    let get_all_duration = (get_all_end - get_all_start) / 1000
    console.log("Get All duration: " + get_all_duration + ' sec')

    for (let i = 0; i < loop_count; i++) {
        const mail_adr = arrived_mail_list[i]
        const mail_result = await alex.call("app", "snapmail", "get_mail", {"address": mail_adr})
        console.log('mail_result ' + i + ' : ' + JSON.stringify(mail_result.Ok))
        const result_obj = mail_result.Ok.mail
        t.match(result_obj.payload, RegExp('.*(' + 'blablabla' + ').*'))
        //t.deepEqual(send_params.payload, result_obj.payload)
    }

    let get_end = Date.now();
    let get_duration = (get_end - get_all_end) / 1000
    console.log("Get duration: " + get_duration + ' sec')

    const received_result = await billy.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result1 : ' + JSON.stringify(received_result.Ok))
    t.deepEqual(received_result.Ok.Err.length, 1)
    t.deepEqual(received_result.Ok.Err[0], alex.info('app').agentAddress)

    // -- ACK -- //

    let ack_duration = 0

    // for (let i = 0; i < loop_count; i++) {
    //     const ack_result = await alex.call("app", "snapmail", "acknowledge_mail", {"inmail_address": arrived_mail_list[i]})
    //     console.log('ack_result ' + i + ' : ' + ack_result.Ok)
    // }
    // await s.consistency()
    // let ack_end = Date.now();
    // ack_duration = (ack_end - get_end) / 1000
    // console.log("Get duration: " + ack_duration + ' sec')
    //
    // const received_result2 = await billy.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    // console.log('received_result2 : ' + JSON.stringify(received_result2.Ok))
    // t.deepEqual(received_result2.Ok.Ok, null)
    //
    // const ack_result2 = await alex.call("app", "snapmail", "has_ack_been_received", {"inmail_address": arrived_mail_list[0]})
    // console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    // t.deepEqual(ack_result2.Ok, true)
    //

    // -- Stats -- //

    let end = Date.now();
    let test_duration = (end - send_start) / 1000

    console.log("\n\n");
    console.log("== Stress single =================== " + count);
    console.log("==================================== " + loop_count);
    console.log("Send duration    : " + send_duration + ' sec')
    console.log("Get All duration : " + get_all_duration + ' sec')
    console.log("Get duration     : " + get_duration + ' sec')
    console.log("Ack duration     : " + ack_duration + ' sec')
    console.log("------------------------------------");
    console.log("Test duration    : " + test_duration + ' sec')
    console.log("====================================");
    console.log("\n");
};
