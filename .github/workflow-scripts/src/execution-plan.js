import { writeFile } from "fs/promises";
import { getOctokit } from "@actions/github";
import { createHash } from "crypto";
import { globSync } from "glob";
import { readFileSync, existsSync } from "fs";
import ignore from "ignore";
import { JOBS, COMPONENTS, tryParseJson } from "./workflow-config.js";
import { dirname, join } from "path";

// Calculate hash for a component's files
function calculateComponentHash(componentConfig) {
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

  // Sort files for consistent hashing
  files.sort().forEach((file) => {
    try {
      const content = readFileSync(file, "utf8");
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

async function getGistFiles(gistId, token) {
  const octokit = getOctokit(token);
  const { data: gist } = await octokit.rest.gists
    .get({ gist_id: gistId })
    .catch((error) => {
      if (error.status === 404) {
        console.warn("No gist found");
        return { data: { files: {} } };
      }
      throw error;
    });

  return gist.files;
}

function getPreviousRun(gistFiles, filename) {
  const gistFile = gistFiles[filename];
  let previousRun = null;
  if (gistFile) {
    previousRun = tryParseJson(gistFile.content);
  }
  return previousRun;
}

function generate_hashes() {
  let component_hashes = {};

  Object.entries(COMPONENTS).forEach(([componentName, componentConfig]) => {
    component_hashes[componentName] = calculateComponentHash(componentConfig);
  });
  return component_hashes;
}

async function generateExecutionPlan() {
  const gistId = process.env.GIST_ID;
  const token = process.env.GIST_SECRET;
  const commitSha = process.env.GITHUB_SHA;

  if (!gistId || !token || !commitSha) {
    throw new Error("Missing required environment variables");
  }

  const gistFiles = await getGistFiles(gistId, token);
  const component_hashes = generate_hashes();

  // Calculate component hashes first
  Object.keys(JOBS).forEach((jobName) => {
    JOBS[jobName].component.hash = component_hashes[jobName];
    JOBS[jobName].filename = `${jobName}.${component_hashes[jobName]}.json`;

    JOBS[jobName].previous_run = getPreviousRun(
      gistFiles,
      JOBS[jobName].filename
    );
  });

  // Generate plan
  const plan = {
    jobs: JOBS,
    metadata: {
      commit_sha: process.env.GITHUB_SHA,
      workflow_id: process.env.GITHUB_WORKFLOW,
      branch: process.env.GITHUB_REF_NAME,
      run_id: process.env.GITHUB_RUN_ID,
      run_number: process.env.GITHUB_RUN_NUMBER,
      repository: process.env.GITHUB_REPOSITORY,
    },
  };

  const planFile = join(".github", "workflow-scripts", "execution-plan.json");
  await writeFile(planFile, JSON.stringify(plan, null, 2));
  console.log("Generated execution plan:", JSON.stringify(plan));

  return plan;
}

async function main() {
  try {
    await generateExecutionPlan();
  } catch (error) {
    console.error("Error generating execution plan:", error);
    process.exit(1);
  }
}

main();
