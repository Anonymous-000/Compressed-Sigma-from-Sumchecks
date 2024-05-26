// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the HyperPlonk library.

// You should have received a copy of the MIT License
// along with the HyperPlonk library. If not, see <https://mit-license.org/>.

//! This module defines structs that are shared by all sub protocols.

use crate::virtual_group_polynomial::VirtualGroupPolynomial;
use ark_ff::PrimeField;
use ark_ec::CurveGroup;
use ark_serialize::CanonicalSerialize;

/// An IOP proof is a collections of
/// - messages from prover to verifier at each round through the interactive
///   protocol.
/// - a point that is generated by the transcript for evaluation
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IOPProof<G: CurveGroup> {
    pub point: Vec<G::ScalarField>,
    pub proofs: Vec<IOPProverMessage<G>>,
}

/// A message from the prover to the verifier at a given round
/// is a list of evaluations.
#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize)]
pub struct IOPProverMessage<G: CurveGroup> {
    pub(crate) evaluations: Vec<G>,
}

/// Prover State of a PolyIOP.
#[derive(Debug)]
pub struct IOPProverState<G: CurveGroup> {
    /// sampled randomness given by the verifier
    pub challenges: Vec<G::ScalarField>,
    /// the current round number
    pub(crate) round: usize,
    /// pointer to the virtual polynomial
    pub(crate) poly: VirtualGroupPolynomial<G>,
    /// points with precomputed barycentric weights for extrapolating smaller
    /// degree uni-polys to `max_degree + 1` evaluations.
    pub(crate) extrapolation_aux: Vec<(Vec<G::ScalarField>, Vec<G::ScalarField>)>,
}

/// Prover State of a PolyIOP
#[derive(Debug)]
pub struct IOPVerifierState<G: CurveGroup> {
    pub(crate) round: usize,
    pub(crate) num_vars: usize,
    pub(crate) max_degree: usize,
    pub(crate) finished: bool,
    /// a list storing the univariate polynomial in evaluation form sent by the
    /// prover at each round
    pub(crate) polynomials_received: Vec<Vec<G>>,
    /// a list storing the randomness sampled by the verifier at each round
    pub(crate) challenges: Vec<G::ScalarField>,
}