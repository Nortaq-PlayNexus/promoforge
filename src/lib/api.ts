import { invoke } from "@tauri-apps/api/core";
import type {
  Project,
  ProductKnowledgeBase,
  Strategy,
  ContentAsset,
  AppSettings,
  Campaign,
} from "./types";

export const api = {
  // Projects
  async getProjects(): Promise<Project[]> {
    return invoke("get_projects");
  },

  async getProject(id: string): Promise<Project> {
    return invoke("get_project", { id });
  },

  async createProject(
    path: string,
    name?: string
  ): Promise<Project> {
    return invoke("create_project", { path, name });
  },

  // Analysis
  async analyzeProject(projectId: string): Promise<ProductKnowledgeBase> {
    return invoke("analyze_project", { projectId });
  },

  async getPkb(projectId: string): Promise<ProductKnowledgeBase> {
    return invoke("get_pkb", { projectId });
  },

  // Strategy
  async generateStrategy(projectId: string): Promise<Strategy> {
    return invoke("generate_strategy", { projectId });
  },

  // Content Generation
  async generateTextAssets(
    projectId: string,
    assetType: string,
    platform: string,
    count: number
  ): Promise<ContentAsset[]> {
    return invoke("generate_text_assets", {
      projectId,
      assetType,
      platform,
      count,
    });
  },

  // Assets
  async approveContent(
    assetId: string,
    approved: boolean
  ): Promise<ContentAsset> {
    return invoke("approve_content", { assetId, approved });
  },

  // Settings
  async getSettings(): Promise<AppSettings> {
    return invoke("get_settings");
  },

  async updateSettings(settings: AppSettings): Promise<void> {
    return invoke("update_settings", { settings });
  },

  // Campaigns
  async getCampaign(id: string): Promise<Campaign> {
    return invoke("get_campaign", { id });
  },

  async listCampaigns(projectId: string): Promise<Campaign[]> {
    return invoke("list_campaigns", { projectId });
  },

  // Export
  async exportPackage(projectId: string): Promise<string> {
    return invoke("export_package", { projectId });
  },

  // Health
  async healthCheck(): Promise<any> {
    return invoke("health_check");
  },
};
