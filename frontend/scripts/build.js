import fs from 'fs'
import esbuild from "esbuild"
import esbuildSvelte from "esbuild-svelte"
import sveltePreprocess from "svelte-preprocess"

if (!fs.existsSync("./dist/")) {
    fs.mkdirSync("./dist/")
}

const args = process.argv.slice(2),
    watch = args.includes("--watch"),
    define = {}

for (const k in process.env) {
  define[`process.env.${k}`] = JSON.stringify(process.env[k])
}

esbuild
    .build({
        entryPoints: ["./src/main.ts"],
        outdir: "./dist",
        format: "esm",
        bundle: true,
        minify: false,
        splitting: true,
        sourcemap: "inline",
        watch: watch,
        plugins: [
            esbuildSvelte({
                preprocess: sveltePreprocess(),
            }),
        ],
        define,
    })
    .catch(() => process.exit(1))

const checkErr = (err) => { if (err) throw err }

fs.copyFile("./index.html", "./dist/index.html", checkErr)

fs.readdir("./public", (err, files) => {
    checkErr(err)
    files.forEach(file => {
        fs.copyFile(`./public/${file}`, `./dist/${file}`, checkErr)
    })
})
