var i = 0
setInterval(function () {
  console.log(++i, 'running')
}, 1000)

process.on('SIGINT', function () {
  console.log('node: GOT SIGINT')
})
