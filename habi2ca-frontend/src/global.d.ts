/// <reference types="@sveltejs/kit" />

declare global {
    interface RequestInit {
        duplex?: 'half' | 'full';
    }
}

// This empty export is necessary to make this a module
export { };
