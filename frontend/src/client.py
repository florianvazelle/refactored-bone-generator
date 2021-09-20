import asyncio

from typing import Callable, List
from grpclib.client import Channel

from .proto.bone_generator import BoneGeneratorStub, Member

class RpcHandler:
    @classmethod
    async def rpc_async_req(cls, in_members: List[Member]) -> List[Member]:
        async with Channel('[::1]', 50052) as channel:
            print("in_members", in_members)
            service = BoneGeneratorStub(channel)
            reply = await service.execute(members=in_members)
            print(reply.members)
            return reply.members