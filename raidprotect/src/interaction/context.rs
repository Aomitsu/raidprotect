//! Interaction context.
//!
//! This module contains types used to parse context from received interaction.

use anyhow::anyhow;
use raidprotect_model::mongodb::guild::{Config, Guild};
use twilight_model::{
    application::interaction::{
        application_command::CommandData,
        message_component::MessageComponentInteractionData,
        modal::{ModalInteractionData, ModalSubmitInteraction},
        ApplicationCommand, MessageComponentInteraction,
    },
    guild::PartialMember,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker},
        Id,
    },
    user::User,
};

use crate::cluster::ClusterState;

/// Context of an [`ApplicationCommand`] or [`MessageComponentInteraction`].
///
/// This type is used for both command and interaction components as the types
/// are similar. A generic parameter is used for the `data` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionContext<D> {
    /// ID of the command.
    pub id: Id<InteractionMarker>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// Token of the command.
    pub token: String,
    /// Data from the invoked command.
    pub data: D,
    /// The channel the command was triggered from.
    pub channel: Id<ChannelMarker>,
    /// If the command was triggered in a guild, the guild context.
    pub guild: Option<GuildContext>,
    /// User that triggered the command.
    pub user: User,
    /// The user locale.
    pub locale: String,
}

impl InteractionContext<CommandData> {
    /// Initialize a new [`InteractionContext`] from an [`ApplicationCommand`].
    pub async fn from_command(
        command: ApplicationCommand,
        state: &ClusterState,
    ) -> Result<Self, anyhow::Error> {
        match command.guild_id {
            Some(guild_id) => Self::from_guild_command(command, state, guild_id).await,
            None => Self::from_private_command(command),
        }
    }

    /// Initialize context from a command that occurred in a guild.
    async fn from_guild_command(
        command: ApplicationCommand,
        state: &ClusterState,
        guild_id: Id<GuildMarker>,
    ) -> Result<Self, anyhow::Error> {
        let member = command
            .member
            .ok_or_else(|| anyhow!("missing member data"))?;
        let user = member
            .user
            .clone()
            .ok_or_else(|| anyhow!("missing user data"))?;

        let guild = state.mongodb().get_guild_or_create(guild_id).await?;

        Ok(Self {
            id: command.id,
            application_id: command.application_id,
            token: command.token,
            data: command.data,
            channel: command.channel_id,
            guild: Some(GuildContext {
                id: guild_id,
                guild,
                member,
            }),
            user,
            locale: command.locale,
        })
    }

    /// Initialize context from a command that occurred in private messages.
    fn from_private_command(command: ApplicationCommand) -> Result<Self, anyhow::Error> {
        let user = command.user.ok_or_else(|| anyhow!("missing user data"))?;

        Ok(Self {
            id: command.id,
            application_id: command.application_id,
            token: command.token,
            data: command.data,
            channel: command.channel_id,
            guild: None,
            user,
            locale: command.locale,
        })
    }
}

impl InteractionContext<MessageComponentInteractionData> {
    /// Initialize a new [`InteractionContext`] from an [`MessageComponentInteraction`].
    pub async fn from_component(
        component: MessageComponentInteraction,
        state: &ClusterState,
    ) -> Result<Self, anyhow::Error> {
        match component.guild_id {
            Some(guild_id) => Self::from_guild_component(component, state, guild_id).await,
            None => Self::from_private_component(component),
        }
    }

    /// Initialize context from a component triggered in a guild.
    ///
    /// The implementation is similar to `from_guild_command`.
    async fn from_guild_component(
        component: MessageComponentInteraction,
        state: &ClusterState,
        guild_id: Id<GuildMarker>,
    ) -> Result<Self, anyhow::Error> {
        let member = component
            .member
            .ok_or_else(|| anyhow!("missing member data"))?;
        let user = member
            .user
            .clone()
            .ok_or_else(|| anyhow!("missing user data"))?;

        let guild = state.mongodb().get_guild_or_create(guild_id).await?;

        Ok(Self {
            id: component.id,
            application_id: component.application_id,
            token: component.token,
            data: component.data,
            channel: component.channel_id,
            guild: Some(GuildContext {
                id: guild_id,
                guild,
                member,
            }),
            user,
            locale: component.locale,
        })
    }

    /// Initialize context from a component triggered in private messages.
    fn from_private_component(
        component: MessageComponentInteraction,
    ) -> Result<Self, anyhow::Error> {
        let user = component.user.ok_or_else(|| anyhow!("missing user data"))?;

        Ok(Self {
            id: component.id,
            application_id: component.application_id,
            token: component.token,
            data: component.data,
            channel: component.channel_id,
            guild: None,
            user,
            locale: component.locale,
        })
    }
}

impl InteractionContext<ModalInteractionData> {
    /// Initialize a new [`InteractionContext`] from an [`ModalSubmitInteraction`].
    pub async fn from_modal(
        modal: ModalSubmitInteraction,
        state: &ClusterState,
    ) -> Result<Self, anyhow::Error> {
        match modal.guild_id {
            Some(guild_id) => Self::from_guild_modal(modal, state, guild_id).await,
            None => Self::from_private_modal(modal),
        }
    }

    /// Initialize context from a component triggered in a guild.
    ///
    /// The implementation is similar to `from_guild_command`.
    async fn from_guild_modal(
        modal: ModalSubmitInteraction,
        state: &ClusterState,
        guild_id: Id<GuildMarker>,
    ) -> Result<Self, anyhow::Error> {
        let member = modal.member.ok_or_else(|| anyhow!("missing member data"))?;
        let user = member
            .user
            .clone()
            .ok_or_else(|| anyhow!("missing user data"))?;

        let guild = state.mongodb().get_guild_or_create(guild_id).await?;

        Ok(Self {
            id: modal.id,
            application_id: modal.application_id,
            token: modal.token,
            data: modal.data,
            channel: modal.channel_id,
            guild: Some(GuildContext {
                id: guild_id,
                guild,
                member,
            }),
            user,
            locale: modal.locale,
        })
    }

    /// Initialize context from a component triggered in private messages.
    fn from_private_modal(modal: ModalSubmitInteraction) -> Result<Self, anyhow::Error> {
        let user = modal.user.ok_or_else(|| anyhow!("missing user data"))?;

        Ok(Self {
            id: modal.id,
            application_id: modal.application_id,
            token: modal.token,
            data: modal.data,
            channel: modal.channel_id,
            guild: None,
            user,
            locale: modal.locale,
        })
    }
}

/// Additional context for commands triggered in a guild.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildContext {
    /// ID of the guild.
    pub id: Id<GuildMarker>,
    /// The guild configuration from database.
    pub guild: Guild,
    /// The member that triggered the command.
    pub member: PartialMember,
}

impl GuildContext {
    /// Get the [`Config`] of the guild.
    pub fn config(&self) -> &Config {
        &self.guild.config
    }
}
