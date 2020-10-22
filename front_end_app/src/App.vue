<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <h1>Welcome to Your Vue.js + TypeScript App</h1>
  </div>
</template>

<script>
import { HelloServiceClient } from '../../proto/output/HelloServiceClientPb'
import { SayHelloRequest } from '../../proto/output/hello_pb'

const hello = new HelloServiceClient('http://0.0.0.0:9091', null, null)
const request = new SayHelloRequest()
const name = 'gRPC!'
request.setName(name)

const metadata = { 'grpc-web-text': '*' }
hello.sayHello(request, metadata, (err, response) => {
  if (err) {
    console.log(err)
  } else {
    console.log(response.getMessage())
  }
})

export default {
  name: 'app'
}

</script>
