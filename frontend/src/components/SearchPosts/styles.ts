import styled from 'styled-components'

export const SearchPostsContainer = styled.form`
  margin: 1.5rem auto;
  position: relative;

  input {
    width: 100%;
    border-radius: 3px;
    border: 0;
    border: 1px solid ${props => props.theme['primary']}70;
    border-bottom: 1px solid ${props => props.theme['primary-400']}40;
    border-right: 1px solid ${props => props.theme['primary-400']}40;
    background: #FFFFFF10;
    backdrop-filter: blur(45px);
    box-shadow: 0 0 25px rgba(0, 0, 0, 0.1), inset 0 0 10px 6px ${p => p.theme['secondary-950']}30;
    color: ${props => props.theme['text']};
    padding: 1rem;

    &::placeholder {
      color: ${props => props.theme['secondary-700']};
    }

    &:disabled {
      cursor: not-allowed;
    }

    &:focus {
      box-shadow: 0 0 25px rgba(0, 0, 0, 0.3), inset 0 0 10px 4px ${p => p.theme['secondary-950']}30;
    }
  }

  button {
    position: absolute;
    right: 0.5rem;
    top: 0.5rem;

    display: flex;
    align-items: center;
    gap: 0.75rem;

    border: 0;
    padding: 0.5rem;
    background: ${props => props.theme['secondary-500']};
    background: linear-gradient(to bottom right, ${props => props.theme['secondary-500']}, ${props => props.theme['secondary-700']});
    color: ${props => props.theme['accent-100']};
    font-weight: bold;
    border-radius: 3px;
    cursor: pointer;

    &:disabled {
      color: ${props => props.theme['text-700']};
      border-color: ${props => props.theme['primary-700']};
      cursor: not-allowed;
    }

    &:not(:disabled):hover {
      background: ${props => props.theme['secondary']};
      background: linear-gradient(to bottom right, ${props => props.theme['secondary-300']}, ${props => props.theme['secondary-500']});
      color: ${props => props.theme['accent-200']};
      box-shadow: 0 0 8px 2px ${props => props.theme['secondary-950']};
      transition: background-color 0.2s, box-shadow 0.1s;
    }
  }
`
