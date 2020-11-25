<template>

  <div id="left">
    <div id="options-container" v-if="isLoaded && optionsOpen">
      <div id="board-inputs">
      <div class="input">
        <span>Seed</span> <input v-model.number="seed" type="number" />
      </div>
      <div class="input">
        <span>Height</span> <input v-model.number="height" type="number" />
      </div>
      <div class="input">
        <span>Width</span> <input v-model.number="width" type="number" />
      </div>
      <div class="input">
        <span>Scale</span> <input v-model.number="scale" type="number" />
      </div>
      </div>
      <div><span>Show</span><span>Step</span></div>
      <div class="button-row">
        <input type="checkbox" v-model="showBuild" />
        <input v-model.number="buildStep" type="number" />
        <button @click="mazeBuild">Build</button></div>
      <div class="button-row">
        <input type="checkbox" v-model="showSolve" />
        <input v-model.number="solveStep" type="number" />
        <button :disabled="false===isBuilt || true === isSolved" @click="mazeSolve">Solve</button>
      </div>

      <section id="color-container">
      <div class="color"><span>Open</span> <input type="color" v-model="colorOpen" /></div>
      <div class="color"><span>Wall</span> <input type="color" v-model="colorWall" /></div>
      <div class="color"><span>Start</span> <input type="color" v-model="colorStart" /></div>
      <div class="color"><span>Finish</span> <input type="color" v-model="colorFinish" /></div>
      <div class="color"><span>Path</span> <input type="color" v-model="colorPath" /></div>
      <div class="color"><span>Solved</span> <input type="color" v-model="colorSolution" /></div>
      </section>

      <button @click="toggleRun">{{ runToggle }}</button>
    </div>
    <div v-else></div>
  </div>
  <div id="main">
    <div id="maze-container">
      <canvas ref="canvas" id="canvas"></canvas>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'

const modWasm = () => import('../maze')

function toParts (index, width) {
  return {
    x: index % width,
    y: Math.floor(index / width)
  }
}

export default {
  name: 'Main',
  computed: {
    runToggle () {
      return this.isRunning ? 'Stop' : 'Run'
    },

    scaledHeight () {
      return this.scale * this.height
    },

    scaledWidth () {
      return this.scale * this.width
    },

    colors () {
      return [
        this.colorWall,
        this.colorOpen,
        this.colorStart,
        this.colorFinish
      ]
    }

  },
  data () {
    return {
      app: null,
      isRunning: false,
      isBuilt: false,
      isSolved: false,
      seed: 1,
      height: 11,
      scale: 3,
      width: 11,
      showBuild: true,
      showSolve: true,
      buildStep: 1,
      solveStep: 1,
      colorOpen: '#FFFFFF',
      colorWall: '#555555',
      colorStart: '#00FF00',
      colorFinish: '#FF0000',
      colorPath: '#FFFF00',
      colorSolution: '#0000AA'
    }
  },
  props: {
    optionsOpen: Boolean
  },
  methods: {
    createMaze () {
      this.app = this.wasm.wasm.App.new(this.seed, this.height, this.width)
      this.isBuilt = false
      this.isSolved = false
      this.fillBoard()
    },

    toggleRun (evt) {
      this.isRunning = !this.isRunning
    },

    mazeBuild (evt) {
      if (this.app === null || this.isBuilt) {
        this.createMaze()
      }
      this.isRunning = true
      const ctx = this.context()
      const mazePtr = this.app.get_board()
      const drawCells = (cells) => {
        const board = new Uint8Array(this.wasm.memory.buffer, mazePtr, this.width * this.height)
        ctx.beginPath()

        cells.map(idx => {
          const parts = toParts(idx, this.width)
          ctx.fillStyle = this.colors[board[idx]]
          ctx.fillRect(
            parts.x * (this.scale),
            parts.y * (this.scale),
            this.scale,
            this.scale
          )
        })

        ctx.stroke()
      }

      const mazeDraw = () => {
        if (!this.isRunning) {
          return
        }

        if (this.app.is_built()) {
          this.isRunning = false
          this.isBuilt = true

          return
        }

        let cells = []

        for (let i = 0; i < this.buildStep; i++) {
          cells = cells.concat(...this.app.build_tick())
        }

        drawCells(cells)
        requestAnimationFrame(mazeDraw)
      }

      requestAnimationFrame(mazeDraw)
    },

    mazeSolve (evt) {
      if (this.app === null || !this.isBuilt) {
        return
      }

      this.isRunning = true
      const ctx = this.context()
      const drawCells = (cells) => {
        ctx.beginPath()

        cells.map(idx => {
          const parts = toParts(idx, this.width)
          ctx.fillStyle = this.colorPath
          ctx.fillRect(
            parts.x * (this.scale),
            parts.y * (this.scale),
            this.scale,
            this.scale
          )
        })

        ctx.stroke()
      }

      const drawSolution = (cells) => {
        ctx.beginPath()

        cells.map(idx => {
          const parts = toParts(idx, this.width)
          ctx.fillStyle = this.colorSolution
          ctx.fillRect(
            parts.x * (this.scale),
            parts.y * (this.scale),
            this.scale,
            this.scale
          )
        })

        ctx.stroke()
      }

      const solve = () => {
        if (!this.isRunning) {
          return
        }

        if (this.showSolve === false) {
          while (this.app.is_solved() === false) {
            if (this.isRunning === false) {
              return
            }

            this.app.solve_tick()
          }
        }

        if (this.app.is_solved()) {
          const cells = this.app.solution()
          drawSolution(cells)
          this.isRunning = false
          this.isSolved = true
          return
        } else {
          let cells = []
          for (let i = 0; i < this.solveStep; i++) {
            cells = cells.concat(...this.app.solve_tick())
          }

          drawCells(cells)
        }

        requestAnimationFrame(solve)
      }

      requestAnimationFrame(solve)
    },

    context () {
      return this.canvas.getContext('2d')
    },

    fillBoard () {
      this.canvas.height = this.scaledHeight
      this.canvas.width = this.scaledWidth
      const ctx = this.context()
      ctx.beginPath()
      ctx.strokeStyle = this.colorWall

      for (let i = 0; i <= this.scaledHeight; i++) {
        ctx.moveTo(0, i)
        ctx.lineTo(this.scaledWidth, i)
      }

      ctx.stroke()
    }
  },
  setup (props, context) {
    const canvas = ref(null)
    const wasm = ref(null)
    const isLoaded = ref(false)

    onMounted(() => {
      Promise.all([
        modWasm().then(mod => { wasm.value = mod.default })
      ]).then(() => { isLoaded.value = true })
    })
    return { isLoaded, canvas, wasm }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

#maze-container {
  overflow: auto;
}

#maze-container {
  padding: 1em;
  position: absolute;
  left: 0;
  bottom: 0;
  top: 0;
  right: 0;
  overflow: auto;
}

#options-container {
  margin-top: 1em;
}

#left {
  background-color: #35303b;
  color: #9fb3be;
  position: relative;

}

ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

#board-inputs {
  margin: 1em 0;
  border-top: 1px solid #605869;
  border-bottom: 1px solid #605869;
  padding: 0.5em 0;
  div {
    display: flex;
    align-items: center;
    padding: 0.1em 1em;
    span {
      display: inline-block;
      padding-right: 1em;
    }

    input {
      display: inline-block;
      width: 8em;
      margin-left: auto;
    }
  }
}

.button-row {
  margin: 0.25em 0;
  display: flex;
  align-items: center;
  padding: 0.1em 1em;

  input {
    &[type="number"] {
      width: 6em;
    }
  }

  button {
    margin-left: auto;
  }
}

#color-container {
  margin: 1em 0;
  border-top: 1px solid #605869;
  border-bottom: 1px solid #605869;
  padding: 0.5em 0;
  .color {
    display: flex;
    align-items: center;
    padding: 0.1em 1em;
    span {
      display: inline-block;
    }

    input {
      margin-left: auto;
    }
  }
}
</style>
