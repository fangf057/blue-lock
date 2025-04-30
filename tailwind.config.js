module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  preflight: false,
  
  plugins: [require("daisyui"),], // 添加 daisyUI 插件
}
