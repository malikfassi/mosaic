import { createHash } from "crypto";
import { globSync } from "glob";
import { readFileSync, existsSync } from "fs";
import ignore from "ignore";
import { join } from "path";

export function calculateComponentHash(componentConfig) {
    console.log("Component config:", componentConfig);
    const { paths } = componentConfig;
    if (paths.length === 1 && paths[0] === ".") {
        return process.env.GITHUB_SHA;
    }
    const hash = createHash("sha256");
    const ig = ignore();

    // Add root .gitignore if exists
    const root_ignore_file = join(process.env.GITHUB_WORKSPACE, ".gitignore");
    if (existsSync(root_ignore_file)) {
        ig.add(readFileSync(root_ignore_file, "utf8"));
    }

    // Get all files matching the patterns
    const files = paths
        .flatMap((pattern) => {
            const matches = globSync(pattern, {
                dot: true,
                nodir: true,
                cwd: process.env.GITHUB_WORKSPACE,
                absolute: false,
            });
            console.log(`Found ${matches.length} files for pattern ${pattern}`);
            return matches;
        })
        .filter((file) => !ig.ignores(file));

    if (files.length === 0) {
        console.warn(`No files found with patterns:`, paths);
    } else {
        console.log(`Found ${files.length} files:`);
    }

    // print current working directory
    console.log(`Current working directory: ${process.cwd()}`);
    console.log(`GITHUB_WORKSPACE: ${process.env.GITHUB_WORKSPACE}`);

    // Sort files for consistent hashing
    files.sort().forEach((file) => {
        try {
            const content = readFileSync(join(process.env.GITHUB_WORKSPACE, file), "utf8");
            hash.update(`${file}:`);
            hash.update(content);
        } catch (error) {
            console.warn(`Warning: Could not read file ${file}: ${error.message}`);
        }
    });

    const result = hash.digest("hex");
    console.log(`Hash: ${result}`);
    return result;
}

export function generateComponentHashes(components) {
    let component_hashes = {};
    Object.entries(components).forEach(([componentName, componentConfig]) => {
        component_hashes[componentName] = calculateComponentHash(componentConfig);
    });
    return component_hashes;
} 