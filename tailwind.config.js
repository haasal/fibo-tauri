/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{ts,svelte}"],
    theme: {
        extend: {
            colors: {
                light: "#5D3D5E",
                lightgray: "#513B51",
                textlight: "#B6B5B6",
                primary: "#3F0E40",
                secondary: "#350D36",
                tertiary: "#361D37",
            },
        },
    },
    plugins: [],
};
