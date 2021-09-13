# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: proto/service.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import List

import betterproto
import grpclib


@dataclass
class Vertex(betterproto.Message):
    x: float = betterproto.double_field(1)
    y: float = betterproto.double_field(2)
    z: float = betterproto.double_field(3)


@dataclass
class Member(betterproto.Message):
    name: str = betterproto.string_field(1)
    vertices: List["Vertex"] = betterproto.message_field(2)


@dataclass
class Members(betterproto.Message):
    members: List["Member"] = betterproto.message_field(1)


class BoneGeneratorStub(betterproto.ServiceStub):
    async def execute(self, *, members: List["Member"] = []) -> Members:
        request = Members()
        if members is not None:
            request.members = members

        return await self._unary_unary(
            "/bone_generator.BoneGenerator/execute",
            request,
            Members,
        )
