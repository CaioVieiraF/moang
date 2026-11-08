import styled from 'styled-components'

export const ReplyCardContainer = styled.li`
  display: flex;
  flex-direction: column;
  gap: 1rem;

  padding: 1.5rem 1.75rem;
  border-radius: 3px;

  background: ${props => props.theme['secondary-400']}05;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  border-bottom: 1px solid ${props => props.theme['secondary-400']}40;
  border-right: 1px solid ${props => props.theme['secondary-400']}40;
  box-shadow: 0 25px 45px rgba(0, 0, 0, .2);
`

export const ReplyHeader = styled.header`
  display: flex;
  align-items: center;
  gap: 0.9rem;
`

export const ReplyAvatar = styled.img`
  width: 2.75rem;
  height: 2.75rem;
  flex-shrink: 0;

  border-radius: 50%;
  object-fit: cover;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  background: ${props => props.theme['secondary-900']};
`

export const ReplyAvatarFallback = styled.div`
  width: 2.75rem;
  height: 2.75rem;
  flex-shrink: 0;

  display: flex;
  align-items: center;
  justify-content: center;

  border-radius: 50%;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  background: ${props => props.theme['secondary-800']};

  font-size: 0.9rem;
  font-weight: bold;
  letter-spacing: 0.04em;
  color: ${props => props.theme['accent-200']};
`

export const ReplyAuthor = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 0;

  strong {
    font-size: 1rem;
    color: ${props => props.theme['accent-100']};
  }

  span {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem;

    font-size: 0.85rem;
    color: ${props => props.theme['secondary-300']};
  }

  .handle {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dot {
    color: ${props => props.theme['secondary-500']};
  }

  .edited {
    font-style: italic;
    color: ${props => props.theme['secondary-500']};
  }
`

export const ReplySummary = styled.p`
  font-size: 0.9rem;
  color: ${props => props.theme['accent-200']};

  padding: 0.5rem 0.75rem;
  border-radius: 3px;
  border-left: 2px solid ${props => props.theme['accent-400']};
  background: ${props => props.theme['secondary-400']}10;
`

export const ReplyBody = styled.div`
  font-size: 1rem;
  line-height: 1.7;
  color: ${props => props.theme['text-100']};
  overflow-wrap: anywhere;

  p {
    margin-bottom: 0.75rem;
  }

  p:last-child {
    margin-bottom: 0;
  }

  a {
    color: ${props => props.theme['accent-300']};
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }

  blockquote {
    margin: 0.75rem 0;
    padding-left: 0.75rem;
    border-left: 2px solid ${props => props.theme['secondary-500']};
    color: ${props => props.theme['text-200']};
  }

  code {
    padding: 0.1rem 0.35rem;
    border-radius: 3px;
    background: ${props => props.theme['secondary-900']};
    font-size: 0.9em;
  }

  pre {
    overflow-x: auto;
    padding: 0.75rem;
    border-radius: 3px;
    background: ${props => props.theme['secondary-900']};
  }

  img {
    max-width: 100%;
    border-radius: 3px;
  }
`

export const ReplyStats = styled.div`
  display: flex;
  align-items: center;
  gap: 1.25rem;

  span {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;

    font-size: 0.8rem;
    color: ${props => props.theme['secondary-300']};
  }

  span:nth-child(1) svg {
    color: ${props => props.theme['accent-400']};
  }

  span:nth-child(2) svg {
    color: ${props => props.theme['text-300']};
  }

  span:nth-child(3) svg {
    color: ${props => props.theme['secondary-300']};
  }
`

export const ReplyFooter = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 0.75rem;

  padding-top: 0.75rem;
  border-top: 1px solid ${props => props.theme['secondary-400']}30;

  a {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;

    font-size: 0.8rem;
    text-decoration: none;
    color: ${props => props.theme['accent-300']};
  }

  a:hover {
    color: ${props => props.theme['accent-100']};
    text-decoration: underline;
  }
`
